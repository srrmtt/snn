use libm::exp;

pub fn lif(ts: i32, ts_1: i32, v_rest: f64, v_mem_old: f64, tao: f64, weights: Vec<f64>) -> f64 {
    let k = -((ts - ts_1) as f64 / tao);

    let exponential = exp(k);

    let v_mem = v_rest + (v_mem_old - v_rest) * exponential;

    let weight = weights.iter().sum::<f64>();
    return v_mem + weight;
}
