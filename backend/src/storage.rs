use memmap2::{MmapMut};
use std::fs::{OpenOptions, File};
use std::io::{self};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

pub struct MemoryMappedStorage {
    file: File,
    mmap: MmapMut,
    data_size: usize,
}

impl MemoryMappedStorage {
    // 创建或打开内存映射文件
    pub fn new(path: &str, initial_size: usize) -> io::Result<Self> {
        // 打开或创建文件
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;
        
        // 获取文件当前大小
        let file_size = file.metadata()?.len() as usize;
        
        // 如果文件大小小于初始大小，扩展文件
        if file_size < initial_size {
            file.set_len(initial_size as u64)?;
        }
        
        // 创建内存映射
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        
        Ok(Self {
            file,
            mmap,
            data_size: initial_size,
        })
    }
    
    // 写入数据
    pub fn write<T: Serialize>(&mut self, data: &T) -> io::Result<()> {
        // 序列化数据
        let serialized = serialize(data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let data_len = serialized.len();
        
        // 确保映射大小足够
        if data_len > self.data_size {
            self.resize(data_len)?;
        }
        
        // 写入数据
        self.mmap[0..data_len].copy_from_slice(&serialized);
        
        // 刷新到磁盘
        self.mmap.flush()?;
        
        Ok(())
    }
    
    // 读取数据
    pub fn read<'de, T: Deserialize<'de>>(&'de self) -> io::Result<T> {
        // 从内存映射中读取数据
        let data = deserialize(&self.mmap[..]).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(data)
    }
    
    // 调整映射大小
    fn resize(&mut self, new_size: usize) -> io::Result<()> {
        // 扩展文件大小
        self.file.set_len(new_size as u64)?;
        
        // 重新创建映射
        self.mmap = unsafe { MmapMut::map_mut(&self.file)? };
        self.data_size = new_size;
        
        Ok(())
    }
}
