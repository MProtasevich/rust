use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    rf: R,
    bytes_read: usize,
    number_of_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats{
            rf: wrapped,
            bytes_read: 0,
            number_of_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.rf
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.number_of_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.number_of_reads += 1;
        let read_bytes = self.rf.read(buf);
        self.bytes_read += read_bytes.as_ref().unwrap_or(&0);
        read_bytes
    }
}

pub struct WriteStats<W> {
    rf: W,
    bytes_written: usize,
    number_of_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            rf: wrapped,
            bytes_written: 0,
            number_of_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.rf
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.number_of_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.bytes_written += buf.len();
        self.number_of_writes += 1;
        self.rf.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.rf.flush()
    }
}
