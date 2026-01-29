#[no_mangle] pub extern "C" fn Li(a:f32,b:f32,t:f32,d:f32)->f32{a+(b-a)*t/d}
#[no_mangle] pub extern "C" fn In(a:f32,b:f32,t:f32,d:f32)->f32{a*t+(b-a)*t*t/(2.0*d)}
#[no_mangle] pub extern "C" fn Sl(a:f32,b:f32,d:f32)->f32{(b-a)/d}
#[no_mangle] pub extern "C" fn Mx(a:f32,b:f32,t:f32)->f32{t*(a+b)/2.0}
#[no_mangle] pub extern "C" fn Wx(x:f32)->f32{(x*2.0*core::f32::consts::PI).sin()}
