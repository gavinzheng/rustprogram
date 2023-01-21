mod circle;
mod rectangle;
mod triangle;


use circle::Circle; 
use rectangle::Rectangle;
use rectangle::square::Square;
use triangle::Triangle;
use triangle::equilateraltriangle::EquilateralTriangle;

fn main() {

    Circle::whoami();
    Rectangle::whoami();
    Square::whoami();
    Triangle::whoami();
    EquilateralTriangle::whoami();
}
