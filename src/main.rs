#[derive(Debug)]
struct Rect {
    h: u32,
    w: u32,
}
impl Rect {
    fn create_rect_hw(height:u32,width:u32)->Rect{
        Rect{h:height,w:width}
    }
    fn create_square(size:u32)->Rect{
        Rect{h:size,w:size}
    }
    fn area(&self) -> u32 {
        self.h * self.w
    }
    fn canhold(&self, r: &Rect) -> bool {
        self.h >= r.h && self.w >= r.w
    }
}
fn main() {
    let s = Rect { w: 4, h: 6 };
    let s2 = Rect { w: 3, h: 5 };
    let s3 = Rect { w: 7, h: 8 };
    let s4=Rect::create_rect_hw(7, 9);
    let s5=Rect::create_square(8);
    println!("{:?}", s);
    println!("area={}", s.area());
    println!("canhold? {}", s.canhold(&s2));
    println!("canhold? {}", s.canhold(&s3));
    println!("area={}",s4.area());
    println!("area={}",s5.area());
}
