use sofa_sys::*;

fn main() {
    unsafe {
        gcrs_to_itrs();
    }
}

fn print_matrix(m: &[[f64; 3]; 3]) {
    println!(
        "\u{250C} {:+.15} {:+.15} {:+.15} \u{2510}",
        m[0][0], m[0][1], m[0][2]
    );
    println!(
        "\u{2502} {:+.15} {:+.15} {:+.15} \u{2502}",
        m[1][0], m[1][1], m[1][2]
    );
    println!(
        "\u{2514} {:+.15} {:+.15} {:+.15} \u{2518}",
        m[2][0], m[2][1], m[2][2]
    );
}

#[allow(unused_mut)]
unsafe fn gcrs_to_itrs() {
    let mut     iy: i32;
    let mut     im: i32;
    let mut     id: i32;
    let mut     ih: i32;
    let mut    min: i32;
    let mut      j: i32;

    let mut    sec: f64;
    let mut     xp: f64;
    let mut     yp: f64;
    let mut   dut1: f64;
    let mut   dx06: f64;
    let mut   dy06: f64;
    let mut djmjd0: f64 = 0.0;
    let mut   date: f64 = 0.0;
    let mut   time: f64;
    let mut    utc: f64;
    let mut    dat: f64 = 0.0;
    let mut    tai: f64;
    let mut     tt: f64;
    let mut    tut: f64;
    let mut    ut1: f64;
    let mut  rc2ti: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut   rpom: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut  rc2it: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut      x: f64 = 0.0;
    let mut      y: f64 = 0.0;
    let mut      s: f64;
    let mut   rc2i: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut    era: f64;

    // UTC
    iy = 2007;
    im = 4;
    id = 5;
    ih = 12;
    min = 0;
    sec = 0.0;

    // Polar motion (arcsec->radians)
    xp = 0.0349282 * DAS2R;
    yp = 0.4833163 * DAS2R;

    // UT1-UTC (s)
    dut1 = -0.072073685;

    // CIP offsets wrt IAU 2006/2000A (mas->radians)
    dx06 =  0.1750 * DMAS2R;
    dy06 = -0.2259 * DMAS2R;

    // TT (MJD)
    j = iauCal2jd(iy, im, id, &mut djmjd0, &mut date);
    if j < 0 { return; }
    time = (60.0 * (60 * ih + min) as f64 + sec) / DAYSEC;
    utc = date + time;
    j = iauDat(iy, im, id, time, &mut dat);
    if j < 0 { return; }
    tai = utc + dat / DAYSEC;
    tt = tai + 32.184 / DAYSEC;
    // UT1
    tut = time + dut1 / DAYSEC;
    ut1 = date + tut;
    println!("TT  = 2400000.5 + {:.15}", tt);
    println!("UT1 = 2400000.5 + {:.15}", ut1);

    // =========================================== //
    // IAU 2006/2000A, CIO based, using X,Y series //
    // =========================================== //

    // CIP and CIO, IAU 2006/2000A
    iauXy06(djmjd0, tt, &mut x, &mut y);
    s = iauS06(djmjd0, tt, x, y);

    // Add CIP corrections
    x += dx06;
    y += dy06;

    // GCRS to CIRS matrix
    iauC2ixys(x, y, s, rc2i.as_mut_ptr());

    // Earth rotation angle
    era = iauEra00(djmjd0 + date, tut);

    // Form celestial-terrestrial matrix (no polar motion yet)
    iauCr(rc2i.as_mut_ptr(), rc2ti.as_mut_ptr());
    iauRz(era, rc2ti.as_mut_ptr());

    // Polar motion matrix (TIRS->ITRS, IERS 2003)
    iauPom00(xp, yp, iauSp00(djmjd0, tt), rpom.as_mut_ptr());

    // Form celestial-terrestrial matrix (including polar motion)
    iauRxr(rpom.as_mut_ptr(), rc2ti.as_mut_ptr(), rc2it.as_mut_ptr());

    println!("X   = {:+.15}", x);
    println!("Y   = {:+.15}", y);
    println!("s   = {:+.9}\"", s * DR2AS);
    println!("NPB matrix, CIO based");
    print_matrix(&rc2i);
    println!("ERA = {:.12}\u{B0}", era * DR2D);
    println!("celestial to terrestrial matrix (no polar motion)");
    print_matrix(&rc2ti);
    println!("celestial to terrestrial matrix");
    print_matrix(&rc2it);
}
