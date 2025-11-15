use orx_concurrent_queue::ConcurrentQueue;
// use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

/// Simple iterator for length-delimited records (big-endian u16 length)
pub struct LengthDelimitedIter<R: BufRead> {
    reader: R,
}

impl<R: BufRead> LengthDelimitedIter<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BufRead> Iterator for LengthDelimitedIter<R> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut len_buf = [0u8; 2];
        match self.reader.read_exact(&mut len_buf) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return None,
            Err(e) => return Some(Err(e)),
        };
        let total_length = u16::from_be_bytes(len_buf) as usize;

        let mut record = vec![0u8; total_length];
        record[..2].copy_from_slice(&len_buf);

        match self.reader.read_exact(&mut record[2..]) {
            Ok(_) => Some(Ok(record)),
            Err(e) => Some(Err(e)),
        }
    }
}

/// Run a concurrent pipeline that reads length-delimited records from an input file,
/// processes them concurrently, and writes them to an output file.
///
/// # Arguments
///
/// * `input_path` - Path to the input file containing length-delimited records
/// * `output_path` - Path to the output file where processed records will be written
/// * `num_processors` - Number of concurrent processor threads to spawn
///
/// # Pipeline Architecture
///
/// The pipeline consists of three stages:
/// 1. **Sequential Reader**: Reads records from the input file and pushes to input queue
/// 2. **Concurrent Processors**: Multiple threads pop from input queue, process, and push to output queue
/// 3. **Sequential Writer**: Pops from output queue and writes records to output file
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `io::Error` if any stage fails.
pub fn run_pipeline(
    input_path: &str,
    output_path: &str,
    num_processors: usize,
) -> io::Result<()> {
    let input_path_owned = input_path.to_owned();
    let output_path_owned = output_path.to_owned();

    let input_queue = ConcurrentQueue::new();
    let output_queue = ConcurrentQueue::new();

    let read_count = AtomicUsize::new(0);
    let written_count = AtomicUsize::new(0);
    let reader_done = AtomicBool::new(false);
    let processors_done = AtomicBool::new(false);

    thread::scope(|s| {
        // Thread: Sequential Reader
        let reader_handle = {
            let input_queue = &input_queue;
            let read_count = &read_count;
            let reader_done = &reader_done;
            let input_path = input_path_owned.clone();
            s.spawn(move || -> io::Result<()> {
                let file = File::open(input_path)?;
                let buf_reader = BufReader::new(file);
                let mut iter = LengthDelimitedIter::new(buf_reader);

                while let Some(record) = iter.next() {
                    let record = record?;
                    input_queue.push(record);
                    read_count.fetch_add(1, Ordering::Relaxed);
                }
                reader_done.store(true, Ordering::Relaxed);
                Ok(())
            })
        };

        // Threads: Concurrent Processors
        let mut processor_handles = Vec::new();
        for _i in 0..num_processors {
            let input_queue = &input_queue;
            let output_queue = &output_queue;
            let reader_done = &reader_done;
            let handle = s.spawn(move || -> io::Result<()> {
                let mut _rng = rand::rng();
                while !reader_done.load(Ordering::Relaxed) || !input_queue.is_empty() {
                    if let Some(record) = input_queue.pop() {
                        // Placeholder: simulate processing (1-5μs)
                        let delay_micros = 0; // rng.random_range(1..=5);
                        thread::sleep(Duration::from_micros(delay_micros));
                        output_queue.push(record);
                    } else {
                        std::hint::spin_loop();
                    }
                }
                Ok(())
            });
            processor_handles.push(handle);
        }

        // Thread: Serial Writer
        let writer_handle = {
            let output_queue = &output_queue;
            let written_count = &written_count;
            let processors_done = &processors_done;
            let output_path = output_path_owned.clone();
            s.spawn(move || -> io::Result<()> {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(output_path)?;
                let mut writer = BufWriter::with_capacity(8192 * 1024, file); // 8MB buffer

                while !processors_done.load(Ordering::Relaxed) || !output_queue.is_empty() {
                    if let Some(record) = output_queue.pop() {
                        writer.write_all(&record)?;
                        written_count.fetch_add(1, Ordering::Relaxed);
                    } else {
                        std::hint::spin_loop();
                    }
                }
                writer.flush()?;
                Ok(())
            })
        };

        reader_handle.join().unwrap()?;
        for h in processor_handles {
            h.join().unwrap()?;
        }
        processors_done.store(true, Ordering::Relaxed);
        writer_handle.join().unwrap()?;
        assert_eq!(
            read_count.load(Ordering::Relaxed),
            written_count.load(Ordering::Relaxed)
        );
        Ok::<(), io::Error>(())
    })?;

    Ok(())
}

// TODO: orx-parallel on input file Iterator and for each push to output q, which listens and writes to file

