extern crate image;
extern crate bit_array;
extern crate typenum;
extern crate rand;

use image::RgbImage;
use bit_array::BitArray;
use typenum::U8;
use elementary::rand::prelude::*;

pub fn create_image(rule: u8){
    let dx = 1028;
    let dy = 1028;
    println!("Using {}", rule);
    let mut img = RgbImage::new(dx, dy);
    
    let mut pattern: [bool; 3] = [false; 3];
    //let mut rng = thread_rng();
    //let random_number = rand::thread_rng().gen_range(0, 1028);
    for i in 0..dx/2{
        img.get_pixel_mut(rand::thread_rng().gen_range(0, 1028) as u32,0).data = [255,255,255];
    }

    println!("Starting to create an image");
    for i in 0..dy-1{
        for j in 1..dx-1{
            //let tSecond = i > 0 && img.get_pixel(j,i-1).data == [255,255,255];
            for k in 0..3{
                pattern[k] = img.get_pixel(j-1+(k as u32),i).data == [255,255,255];
            }
            if check_rule(&pattern, rule) ^ (i > 0 && img.get_pixel(j,i-1).data == [255,255,255]){
                img.get_pixel_mut(j,i+1).data = [255,255,255];
            };
        }
    }
    img.save(format!("rule{}.png",rule)).unwrap();

}

fn check_rule(pattern: &[bool], rule: u8)-> bool{

    //let new_state = pattern[0]^(pattern[1]||pattern[2]);
    let mut bit_pattern = BitArray::<u32, U8>::new();
    for i in 0..pattern.len(){
        let index = bit_pattern.len()-pattern.len()+i;
        bit_pattern.set(index,pattern[i]);
    }

    /*println!("{:?}",pattern);
    println!("{:?}",bit_pattern);
    println!("Bit  pattern {}",bit_pattern.to_bytes()[0]);*/
    let bit_index = bit_pattern.to_bytes()[0] as usize;
    //println!("Index {}",bit_index);
    let bit_array_rule = BitArray::<u32, U8>::from_bytes(&[rule]);
    //println!("{:?}", bit_array_rule);
    return bit_array_rule[bit_array_rule.len()-1-bit_index];
}

#[cfg(test)]
mod tests {
    #[test]
    fn rule_checking() {
        assert_eq!(true, super::check_rule(&[true,false,false],30));
        assert_eq!(true, super::check_rule(&[false,true,false],30));
        assert_eq!(true, super::check_rule(&[false,false,true],30));
        assert_eq!(false, super::check_rule(&[true,true,true],30));
    }
}
