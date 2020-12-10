use sofa_sys::*;

fn main() {
    unsafe {
        println!("Original values from SOFA tools for Earth Attitude 5.6:");
        gcrs_to_itrs(
            // UTC
            2007,
            4,
            5,
            12,
            0,
            0.0,
            // Polar motion (arcsec->radians)
            0.0349282 * DAS2R,
            0.4833163 * DAS2R,
            // UT1-UTC (s)
            -0.072073685,
            // CIP offsets wrt IAU 2006/2000A (mas->radians)
            0.1750 * DMAS2R,
            -0.2259 * DMAS2R,
        );
        println!("\nDifferent values:");
        // The polar motion xp,yp can be obtained from IERS bulletins.  The
        // values are the coordinates (in radians) of the Celestial
        // Intermediate Pole with respect to the International Terrestrial
        // Reference System (see IERS Conventions 2003), measured along the
        // meridians 0 and 90 deg west respectively.  For many
        // applications, xp and yp can be set to zero.
        // https://www.iers.org/IERS/EN/Publications/Bulletins/bulletins.html
        // https://datacenter.iers.org/data/latestVersion/6_BULLETIN_A_V2013_016.txt
        // https://datacenter.iers.org/data/latestVersion/207_BULLETIN_B207.txt
        gcrs_to_itrs(
            2020,
            10,
            10,
            10,
            10,
            10.0,
            184.598 * DMAS2R,
            314.938 * DMAS2R,
            -0.1703470,
            0.083,
            -0.020,
        );
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
unsafe fn gcrs_to_itrs(
    iy: i32,
    im: i32,
    id: i32,
    ih: i32,
    min: i32,
    sec: f64,
    xp: f64,
    yp: f64,
    dut1: f64,
    dx06: f64,
    dy06: f64,
) {
    let mut j: i32;

    let mut djmjd0: f64 = 0.0;
    let mut date: f64 = 0.0;
    let mut time: f64;
    let mut utc: f64;
    let mut dat: f64 = 0.0;
    let mut tai: f64;
    let mut tt: f64;
    let mut tut: f64;
    let mut ut1: f64;
    let mut rc2ti: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut rpom: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut rc2it: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64;
    let mut rc2i: [[f64; 3]; 3] = [[0.0; 3]; 3];
    let mut era: f64;

    // TT (MJD)
    j = iauCal2jd(iy, im, id, &mut djmjd0, &mut date);
    if j < 0 {
        return;
    }
    time = (60.0 * (60 * ih + min) as f64 + sec) / DAYSEC;
    utc = date + time;
    j = iauDat(iy, im, id, time, &mut dat);
    if j < 0 {
        return;
    }
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
