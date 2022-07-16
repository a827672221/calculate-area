fn main() {
  // 声明三个结构体
  let s1 = Triangle {
    bottom: 5.0,
    height: 5.0,
  };
  let s2 = Rectangle {
    width: 5.0,
    height: 5.0,
  };
  let s3 = Circle {
    radius: 5.0
  };
  // 三个结构体调用calculate_area方法计算面积
  calculate_area(&s1);
  calculate_area(&s2);
  calculate_area(&s3);
}
// 定义trait Summary
pub trait Summary {
  // 必须实现calculate方法
  fn calculate(&self) -> f64;
}
// 定义三角形结构体Triangle
pub struct Triangle {
  bottom: f64,
  height: f64,
}
// 定义矩形结构体Rectangle
pub struct Rectangle {
  width: f64,
  height: f64,
}
// 定义圆形结构体Circle
pub struct Circle {
  radius: f64,
}
// 为Triangle结构体实现一个trait Summary
impl Summary for Triangle {
  //定义calculate方法计算面积
  fn calculate(&self) -> f64 {
    (self.bottom * self.height) / 2.0
  }
}
// 为Rectangle结构体实现一个trait Summary
impl Summary for Rectangle {
  //定义calculate方法计算面积
  fn calculate(&self) -> f64 {
    self.width * self.height
  }
}
// 为Circle结构体实现一个trait Summary
impl Summary for Circle {
  //定义calculate方法计算面积
  fn calculate(&self) -> f64 {
    std::f64::consts::PI * self.radius.powf(2.0)
  }
}
// 定义calculate_area方法计算面积
pub fn calculate_area<T: Summary>(item: &T) {
  // 打印面积
  println!("{}", item.calculate());
}