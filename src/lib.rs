use wasm_bindgen::prelude::*;

// Solve a Cubic Equation which has two complex roots

#[wasm_bindgen]
pub fn solve(params: &str) -> String {
  let co: (f32, f32, f32, f32) = serde_json::from_str(&params).unwrap();
  
  let Q: f32 = (((3*co.3*co.1)-(co.2.pow(2)))/(9*(co.3.pow(2))));
  let R: f32 = (((9*co.3*co.2*co.1)-(27*(co.3.pow(2))*co.0)-(2*(co.2.pow(3))))/(54*(co.3.pow(3))));
  let tmp: f32 = Q.pow(3) + R.pow(2);
  let tmp1: f32 = 3./4.;
  
  let mut S: f32 = 0.;
  let mut T: f32 = 0.;
  let mut img: f32 = 0.;
  let mut real: f32 = 0.;
  let mut val1: f32 = (R+tmp.sqrt());
  let mut val2: f32 = (R-tmp.sqrt());
  let mut solution: (f32, f32) = (0., 0.);
  
  if val1 < 0.
  {
    val1 = -1. * val1;
    S = val1.pow(1./3.);
    S = -1. * S;
  }
  else
  {
    S = val1.pow(1./3.);
  }

  if val2 < 0.
  {
    val2 = -1. * val2;
    T = val2.pow(1./3.);
    T = -1. * T;
  }
  else
  {
    T = val2.pow(1./3.);
  }
  
}

real = (-((S+T)/2)-(co[2]/(3*co[3])))
img = ((sqrt(3/4))*(S-T))

x1 = S+T-(co[2]/(3*co[3]))
x2 = complex(real, img)
x3 = complex(real, -img)

print("The solutions for the Equation are: ")
print(x1, x2, x3)
