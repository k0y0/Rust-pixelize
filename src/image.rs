use std::io;

use crate::{error::Error, pixel::Pixel};

#[derive(Debug)]

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

impl Image {    
    const FARBFELD: &'static[u8; 8] = b"farbfeld";

    pub fn new(width: u32, height: u32) -> Self
    {
        let pix_num = width as usize * height as usize;

        let pixels = (0..pix_num).map(|_| {
            Pixel {r: 0, g: 0, b: 0, a: 0xFFFF}
        }).collect();

        Image { width, height, pixels }
    }

    pub fn decode<R>(mut reader: R) -> Result<Self, Error>
    where
        R: io::Read,
    {
        let mut buf = Vec::new();

        reader.read_to_end(&mut buf)?;

        if buf.len() < 16 {
            return Err(Error::HeaderToShort);
        }
        
        if &buf[..8] != Self::FARBFELD {
            return Err(Error::FarbfeldPatternNotFound);
        }

        let width = u32::from_be_bytes(buf[8..12].try_into().unwrap());
        let height = u32::from_be_bytes(buf[12..16].try_into().unwrap());

        let pix_num = width as usize * height as usize;

        let pixels: Vec<Pixel> = buf[16..]
            .chunks_exact(8)
            .take(pix_num)
            .map(|i| {
                let r = u16::from_be_bytes(i[..2].try_into().unwrap());
                let g = u16::from_be_bytes(i[2..4].try_into().unwrap());
                let b = u16::from_be_bytes(i[4..6].try_into().unwrap());
                let a = u16::from_be_bytes(i[6..8].try_into().unwrap());

                Pixel { r, g, b, a }
            })
            .collect();

        if pix_num != pixels.len() {
            return Err(Error::MismatchedLength)
        }
            
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn encode<W>(&self ,mut writer: W) -> Result<(), Error>
    where
        W: io::Write,
    {
        writer.write_all(Self::FARBFELD)?;
        writer.write_all(self.width.to_be_bytes().as_slice())?;
        writer.write_all(self.height.to_be_bytes().as_slice())?;

        for pixel in self.pixels.iter() {
            writer.write_all(pixel.r.to_be_bytes().as_slice())?;
            writer.write_all(pixel.g.to_be_bytes().as_slice())?;
            writer.write_all(pixel.b.to_be_bytes().as_slice())?;
            writer.write_all(pixel.a.to_be_bytes().as_slice())?;
        }

        Ok(())
    }


    pub fn pixelize(&self, merge_value: u32) -> Result<Self, Error> {

        if self.width % merge_value != 0 || self.height % merge_value != 0 {
            return Err(Error::NonExactMultiple);
        }  
        
        let width = self.width / merge_value;
        let height = self.height / merge_value; 
        
        let mut new_image = Self::new(width, height);
        let merge_items = merge_value as u64 * merge_value as u64;
        for x in 0..width as usize {
            
            for y in 0..height as usize {
                
                let mut new: (u64,u64,u64,u64) = (0,0,0,0);
                
                for i_x in (x * merge_value as usize)..(x*merge_value as usize) + merge_value as usize  {
                
                    for i_y in (y * merge_value as usize)..(y * merge_value as usize) + merge_value as usize  {

                        let index = i_x + (i_y * self.width as usize);
                        let Pixel{r,g,b,a} = self.pixels.get( index ).unwrap();
                        
                        new.0 += *r as u64;
                        new.1 += *g as u64;
                        new.2 += *b as u64;
                        new.3 += *a as u64;
                    }
                }

                let p = new_image.pixels.get_mut(x + ( y * width as usize )).unwrap();

                p.r = (new.0 / ( merge_items )) as u16;
                p.g = (new.1 / ( merge_items )) as u16;
                p.b = (new.2 / ( merge_items )) as u16;
                p.a = (new.3 / ( merge_items )) as u16;
            
            }
        }
        
        Ok(new_image)
    }
}
