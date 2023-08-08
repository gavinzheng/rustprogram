struct Decoder<'a, 'b, S, R> {
  schema: &'a S,
  reader: &'b R
}
  
impl<'a, 'b, S, R> Decoder<'a, 'b, S, R>
where 'a: 'b { //  'a outlives 'b，或者说'b不能比'a活得长.
}
  
fn main() {
  let a: Vec<u8> = vec![];
  let b: Vec<u8> = vec![];
  let decoder = Decoder {schema: &a, reader: &b};
}