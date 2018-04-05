#[derive(Debug)]
pub enum Expr {
  QReg(String, i32),
  CReg(String, i32),
}

#[derive(Debug)]
pub struct Program {
  /// The version of OPENQASM to use, this only supports version 2.0, so an error will be 
  /// thrown if the version is not 2.0
  pub version: f32, 
  pub body: Vec<Expr>,
}