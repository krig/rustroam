use std::f32;

pub type Scalar = f32;

const PI: Scalar = f32::consts::PI;
const FRAC_PI_2: Scalar = f32::consts::FRAC_PI_2;

pub fn ease(a: Scalar, b: Scalar, t: Scalar) -> Scalar {
    a + (b - a) * t
}


pub fn linear(t: Scalar) -> Scalar {
    t
}


pub fn quadratic_in(t: Scalar) -> Scalar {
    t * t
}


pub fn quadratic_out(t: Scalar) -> Scalar {
    -(t * (t - 2.))
}


pub fn quadratic_in_out(t: Scalar) -> Scalar {
    if t < 0.5 {
	    2. * t * t
    } else {
        (-2. * t * t) + (4. * t) - 1.
    }
}


pub fn cubic_in(t: Scalar) -> Scalar {
	t * t * t
}


pub fn cubic_out(t: Scalar) -> Scalar {
    let f = t - 1.;
    f * f * f + 1.
}


pub fn cubic_in_out(t: Scalar) -> Scalar {
	if t < 0.5 {
		4. * t * t * t
	} else {
		let f = (2. * t) - 2.;
		0.5 * f * f * f + 1.
	}
}


pub fn quartic_in(t: Scalar) -> Scalar {
    t * t * t * t
}


pub fn quartic_out(t: Scalar) -> Scalar {
	let f = t - 1.;
	f * f * f * (1. - t) + 1.
}


pub fn quartic_in_out(t: Scalar) -> Scalar {
	if t < 0.5 {
		8. * t * t * t * t
	} else {
		let f = t - 1.;
		-8. * f * f * f * f + 1.
	}
}


pub fn quintic_in(t: Scalar) -> Scalar {
    t * t * t * t * t
}


pub fn quintic_out(t: Scalar) -> Scalar {
	let f = t - 1.;
	f * f * f * f * f + 1.
}


pub fn quintic_in_out(t: Scalar) -> Scalar {
	if t < 0.5 {
		16. * t * t * t * t * t
	} else {
		let f = (2. * t) - 2.;
		0.5 * f * f * f * f * f + 1.
	}
}


pub fn sine_in(t: Scalar) -> Scalar {
	((t - 1.) * FRAC_PI_2).sin() + 1.
}


pub fn sine_out(t: Scalar) -> Scalar {
	(t * FRAC_PI_2).sin()
}


pub fn sine_in_out(t: Scalar) -> Scalar {
	0.5 * (1. - (t * PI).cos())
}


pub fn circular_in(t: Scalar) -> Scalar {
	1. - (1. - (t * t)).sqrt()
}


pub fn circular_out(t: Scalar) -> Scalar {
	((2. - t) * t).sqrt()
}


pub fn circular_in_out(t: Scalar) -> Scalar {
	if t < 0.5 {
		0.5 * (1. - (1. - 4. * (t * t)).sqrt())
	} else {
		0.5 * ((-((2. * t) - 3.) * ((2. * t) - 1.)).sqrt() + 1.)
	}
}


pub fn exponential_in(t: Scalar) -> Scalar {
	if t <= 0. {
        t
    } else {
        2.0_f32.powf(10. * (t - 1.))
    }
}


pub fn exponential_out(t: Scalar) -> Scalar {
	if t >= 1. {
        t
    } else {
        1. - 2.0_f32.powf(-10. * t)
    }
}


pub fn exponential_in_out(t: Scalar) -> Scalar {
	if t <= 0. || t >= 1. {
		t
    } else if t < 0.5 {
		0.5 * 2.0_f32.powf((20. * t) - 10.)
	} else {
		-0.5 * 2.0_f32.powf((-20. * t) + 10.) + 1.
	}
}


pub fn elastic_in(t: Scalar) -> Scalar {
	(13. * FRAC_PI_2 * t).sin() * 2.0_f32.powf(10. * (t - 1.))
}


pub fn elastic_out(t: Scalar) -> Scalar {
	(-13. * FRAC_PI_2 * (t + 1.)).sin() * 2.0_f32.powf(-10. * t) + 1.
}


pub fn elastic_in_out(t: Scalar) -> Scalar {
	if t < 0.5 {
		0.5 * (13. * FRAC_PI_2 * (2. * t)).sin() * 2.0_f32.powf(10. * ((2. * t) - 1.))
	} else {
		0.5 * ((-13. * FRAC_PI_2 * ((2. * t - 1.) + 1.)).sin() * 2.0_f32.powf(-10. * (2. * t - 1.)) + 2.)
	}
}


pub fn back_in(t: Scalar) -> Scalar {
	t * t * t - t * (t * PI).sin()
}


pub fn back_out(t: Scalar) -> Scalar {
	let f = 1. - t;
	1. - (f * f * f - f * (f * PI).sin())
}


pub fn back_in_out(t: Scalar) -> Scalar {
	if t < 0.5 {
		let f = 2. * t;
		0.5 * (f * f * f - f * (f * PI).sin())
	} else {
		let f = 1. - (2.*t - 1.);
		0.5 * (1. - (f * f * f - f * (f * PI).sin())) + 0.5
	}
}


pub fn bounce_out(t: Scalar) -> Scalar {
	if t < 4. / 11. {
		(121. * t * t) / 16.
	} else if t < 8. / 11. {
		(363. / 40. * t * t) - (99. / 10. * t) + 17. / 5.
	} else if t < 9. / 10. {
		(4356. / 361. * t * t) - (35442. / 1805. * t) + 16061. / 1805.
	} else {
		(54. / 5. * t * t) - (513. / 25. * t) + 268. / 25.
	}
}


pub fn bounce_in(t: Scalar) -> Scalar {
	1. - bounce_out(1. - t)
}


pub fn bounce_in_out(t: Scalar) -> Scalar {
	if t < 0.5 {
		0.5 * bounce_in(t * 2.)
	} else {
		0.5 * bounce_out(t * 2. - 1.) + 0.5
	}
}


pub fn perlin_in_out(t: Scalar) -> Scalar {
	let t3 = t * t * t;
	let t4 = t3 * t;
	let t5 = t4 * t;
	6. * t5 - 15. * t4 + 10. * t3
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_all() {
        assert_eq!(0.0, ease(0.0, 1.0, 0.0));
        assert_eq!(0.0, linear(0.0));
        assert_eq!(0.0, quadratic_in(0.0));
        assert_eq!(0.0, quadratic_out(0.0));
        assert_eq!(0.0, quadratic_in_out(0.0));
        assert_eq!(0.0, cubic_in(0.0));
        assert_eq!(0.0, cubic_out(0.0));
        assert_eq!(0.0, cubic_in_out(0.0));
        assert_eq!(0.0, quartic_in(0.0));
        assert_eq!(0.0, quartic_out(0.0));
        assert_eq!(0.0, quartic_in_out(0.0));
        assert_eq!(0.0, quintic_in(0.0));
        assert_eq!(0.0, quintic_out(0.0));
        assert_eq!(0.0, quintic_in_out(0.0));
        assert_eq!(0.0, sine_in(0.0));
        assert_eq!(0.0, sine_out(0.0));
        assert_eq!(0.0, sine_in_out(0.0));
        assert_eq!(0.0, circular_in(0.0));
        assert_eq!(0.0, circular_out(0.0));
        assert_eq!(0.0, circular_in_out(0.0));
        assert_eq!(0.0, exponential_in(0.0));
        assert_eq!(0.0, exponential_out(0.0));
        assert_eq!(0.0, exponential_in_out(0.0));
        assert_eq!(0.0, elastic_in(0.0));
        assert_eq!(0.0, elastic_out(0.0));
        assert_eq!(0.0, elastic_in_out(0.0));
        assert_eq!(0.0, back_in(0.0));
        assert_eq!(1.0, back_out(1.0));
        assert_eq!(0.0, back_in_out(0.0));
        assert_eq!(0.0, bounce_in(0.0));
        assert_eq!(0.0, bounce_out(0.0));
        assert_eq!(0.0, bounce_in_out(0.0));
        assert_eq!(0.103515625, perlin_in_out(0.25));
    }
}
