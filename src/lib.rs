#![no_std]
#[panic_handler]fn p(_:&core::panic::PanicInfo)->!{loop{}}

const K: f32 = 0.6072529350; // Constante de gain CORDIC
const ANGLES: [f32; 24] = [
    0.785398163, 0.463647609, 0.244978663, 0.124354995, 0.06241881, 0.031239833,
    0.015623729, 0.007812341, 0.00390623, 0.001953123, 0.000976562, 0.000488281,
    0.000244141, 0.00012207, 0.000061035, 0.000030518, 0.000015259, 0.000007629,
    0.000003815, 0.000001907, 0.000000954, 0.000000477, 0.000000238, 0.000000119
];

#[no_mangle]
pub extern "C" fn Wx(x: f32) -> f32 {
    let mut v = x - (x as i32) as f32;
    if v < 0.0 { v += 1.0; }
    
    // Conversion en radians et gestion des quadrants
    let mut target = v * 6.283185307;
    let mut x_c = K;
    let mut y_c = 0.0;
    
    // Ajustement pré-rotation pour couvrir [0, 2pi]
    if target > 1.570796327 && target <= 4.71238898 {
        target -= 3.14159265;
        x_c = -K;
    } else if target > 4.71238898 {
        target -= 6.283185307;
    }

    for i in 0..24 {
        let dx = x_c / (1 << i) as f32;
        let dy = y_c / (1 << i) as f32;
        if target > 0.0 {
            x_c -= dy;
            y_c += dx;
            target -= ANGLES[i];
        } else {
            x_c += dy;
            y_c -= dx;
            target += ANGLES[i];
        }
    }
    y_c
}

#[no_mangle]#[inline(always)]pub extern "C" fn Li(a:f32,b:f32,t:f32,d:f32)->f32{a+(b-a)*t/d}
#[no_mangle]#[inline(always)]pub extern "C" fn In(a:f32,b:f32,t:f32,d:f32)->f32{a*t+(b-a)*t*t/(2.*d)}
#[no_mangle]#[inline(always)]pub extern "C" fn Sl(a:f32,b:f32,d:f32)->f32{(b-a)/d}
#[no_mangle]#[inline(always)]pub extern "C" fn Mx(a:f32,b:f32,d:f32)->f32{d*(a+b)/2.}
