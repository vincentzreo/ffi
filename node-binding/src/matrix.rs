use algo::{multiply, Matrix};
use napi::{Env, JsUnknown, Status};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(js_name = "Matrix")]
#[derive(Debug, Serialize, Deserialize)]
pub struct JsMatrix {
    inner: Matrix<f64>,
}

#[napi]
impl JsMatrix {
    #[napi(constructor)]
    pub fn try_new(data: Vec<Vec<f64>>, _env: Env) -> napi::Result<Self> {
        if data.is_empty() || data[0].is_empty() {
            return Err(napi::Error::new(Status::InvalidArg, "row or col is empty"));
        }
        let row = data.len();
        let col = data[0].len();
        let data = data.into_iter().flatten().collect::<Vec<_>>();
        Ok(Self {
            inner: Matrix::new(data, row, col),
        })
    }

    #[napi]
    pub fn multiply(&self, other: JsUnknown, env: Env) -> napi::Result<Self> {
        let other = if let Ok(true) = other.is_array() {
            let other: Vec<Vec<f64>> = env.from_js_value(other)?;
            JsMatrix::try_new(other, env)?
        } else {
            // have some bug here
            env.from_js_value(other)?
        };
        let result = multiply(&self.inner, &other.inner).unwrap();
        Ok(JsMatrix { inner: result })
    }

    #[napi]
    pub fn display(&self) -> String {
        format!("{}", self.inner)
    }
}
