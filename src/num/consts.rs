pub trait IntConsts {
    static pure fn zero() -> self;
    static pure fn one()  -> self;
}

pub trait FloatConsts: IntConsts {
    static pure fn two_pi()         -> self;    // 2 * π
    static pure fn pi()             -> self;    // π
    static pure fn pi_2()           -> self;    // π / 2
    static pure fn pi_3()           -> self;    // π / 3
    static pure fn pi_4()           -> self;    // π / 4
    static pure fn pi_6()           -> self;    // π / 6
    static pure fn pi_8()           -> self;    // π / 8
    static pure fn frac_pi_2()      -> self;
    static pure fn frac_pi_4()      -> self;
    static pure fn frac_1_pi()      -> self;
    static pure fn frac_2_pi()      -> self;
    static pure fn frac_2_sqrtpi()  -> self;
    static pure fn sqrt2()          -> self;
    static pure fn frac_1_sqrt2()   -> self;
    static pure fn e()              -> self;
    static pure fn log2_e()         -> self;
    static pure fn log10_e()        -> self;
    static pure fn ln_2()           -> self;
    static pure fn ln_10()          -> self;
}

pub impl u8: IntConsts {
    #[inline(always)] static pure fn zero()  -> u8 { 0u8 }
    #[inline(always)] static pure fn one()   -> u8 { 1u8 }
}

pub impl u16: IntConsts {
    #[inline(always)] static pure fn zero()  -> u16 { 0u16 }
    #[inline(always)] static pure fn one()   -> u16 { 1u16 }
}

pub impl u32: IntConsts {
    #[inline(always)] static pure fn zero()  -> u32 { 0u32 }
    #[inline(always)] static pure fn one()   -> u32 { 1u32 }
}

pub impl u64: IntConsts {
    #[inline(always)] static pure fn zero()  -> u64 { 0u64 }
    #[inline(always)] static pure fn one()   -> u64 { 1u64 }
}

pub impl uint: IntConsts {
    #[inline(always)] static pure fn zero()  -> uint { 0u }
    #[inline(always)] static pure fn one()   -> uint { 1u }
}

pub impl i8: IntConsts {
    #[inline(always)] static pure fn zero()  -> i8 { 0i8 }
    #[inline(always)] static pure fn one()   -> i8 { 1i8 }
}

pub impl i16: IntConsts {
    #[inline(always)] static pure fn zero()  -> i16 { 0i16 }
    #[inline(always)] static pure fn one()   -> i16 { 1i16 }
}

pub impl i32: IntConsts {
    #[inline(always)] static pure fn zero()  -> i32 { 0i32 }
    #[inline(always)] static pure fn one()   -> i32 { 1i32 }
}

pub impl i64: IntConsts {
    #[inline(always)] static pure fn zero()  -> i64 { 0i64 }
    #[inline(always)] static pure fn one()   -> i64 { 1i64 }
}

pub impl int: IntConsts {
    #[inline(always)] static pure fn zero()  -> int { 0 }
    #[inline(always)] static pure fn one()   -> int { 1 }
}

pub impl f32: IntConsts {
    #[inline(always)] static pure fn zero()             -> f32 { 0f32 }
    #[inline(always)] static pure fn one()              -> f32 { 1f32 }
}

pub impl f32: FloatConsts {
    #[inline(always)] static pure fn two_pi()           -> f32 { 6.28318530717958647692528676655900576_f32  }
    #[inline(always)] static pure fn pi()               -> f32 { 3.14159265358979323846264338327950288_f32  }
    #[inline(always)] static pure fn pi_2()             -> f32 { 1.57079632679489661923132169163975144_f32  }
    #[inline(always)] static pure fn pi_3()             -> f32 { 1.04719755119659774615421446109316763_f32  }
    #[inline(always)] static pure fn pi_4()             -> f32 { 0.78539816339744830961566084581987572_f32  }
    #[inline(always)] static pure fn pi_6()             -> f32 { 0.52359877559829887307710723054658381_f32  }
    #[inline(always)] static pure fn pi_8()             -> f32 { 0.39269908169872415480783042290993786_f32  }
    #[inline(always)] static pure fn frac_pi_2()        -> f32 { 1.57079632679489661923132169163975144_f32  }
    #[inline(always)] static pure fn frac_pi_4()        -> f32 { 0.785398163397448309615660845819875721_f32 }
    #[inline(always)] static pure fn frac_1_pi()        -> f32 { 0.318309886183790671537767526745028724_f32 }
    #[inline(always)] static pure fn frac_2_pi()        -> f32 { 0.636619772367581343075535053490057448_f32 }
    #[inline(always)] static pure fn frac_2_sqrtpi()    -> f32 { 1.12837916709551257389615890312154517_f32  }
    #[inline(always)] static pure fn sqrt2()            -> f32 { 1.41421356237309504880168872420969808_f32  }
    #[inline(always)] static pure fn frac_1_sqrt2()     -> f32 { 0.707106781186547524400844362104849039_f32 }
    #[inline(always)] static pure fn e()                -> f32 { 2.71828182845904523536028747135266250_f32  }
    #[inline(always)] static pure fn log2_e()           -> f32 { 1.44269504088896340735992468100189214_f32  }
    #[inline(always)] static pure fn log10_e()          -> f32 { 0.434294481903251827651128918916605082_f32 }
    #[inline(always)] static pure fn ln_2()             -> f32 { 0.693147180559945309417232121458176568_f32 }
    #[inline(always)] static pure fn ln_10()            -> f32 { 2.30258509299404568401799145468436421_f32  }
}

pub impl f64: IntConsts {
    #[inline(always)] static pure fn zero()             -> f64 { 0f64 }
    #[inline(always)] static pure fn one()              -> f64 { 1f64 }
}

pub impl f64: FloatConsts {
    #[inline(always)] static pure fn two_pi()           -> f64 { 6.28318530717958647692528676655900576_f64  }
    #[inline(always)] static pure fn pi()               -> f64 { 3.14159265358979323846264338327950288_f64  }
    #[inline(always)] static pure fn pi_2()             -> f64 { 1.57079632679489661923132169163975144_f64  }
    #[inline(always)] static pure fn pi_3()             -> f64 { 1.04719755119659774615421446109316763_f64  }
    #[inline(always)] static pure fn pi_4()             -> f64 { 0.78539816339744830961566084581987572_f64  }
    #[inline(always)] static pure fn pi_6()             -> f64 { 0.52359877559829887307710723054658381_f64  }
    #[inline(always)] static pure fn pi_8()             -> f64 { 0.39269908169872415480783042290993786_f64  }
    #[inline(always)] static pure fn frac_pi_2()        -> f64 { 1.57079632679489661923132169163975144_f64  }
    #[inline(always)] static pure fn frac_pi_4()        -> f64 { 0.785398163397448309615660845819875721_f64 }
    #[inline(always)] static pure fn frac_1_pi()        -> f64 { 0.318309886183790671537767526745028724_f64 }
    #[inline(always)] static pure fn frac_2_pi()        -> f64 { 0.636619772367581343075535053490057448_f64 }
    #[inline(always)] static pure fn frac_2_sqrtpi()    -> f64 { 1.12837916709551257389615890312154517_f64  }
    #[inline(always)] static pure fn sqrt2()            -> f64 { 1.41421356237309504880168872420969808_f64  }
    #[inline(always)] static pure fn frac_1_sqrt2()     -> f64 { 0.707106781186547524400844362104849039_f64 }
    #[inline(always)] static pure fn e()                -> f64 { 2.71828182845904523536028747135266250_f64  }
    #[inline(always)] static pure fn log2_e()           -> f64 { 1.44269504088896340735992468100189214_f64  }
    #[inline(always)] static pure fn log10_e()          -> f64 { 0.434294481903251827651128918916605082_f64 }
    #[inline(always)] static pure fn ln_2()             -> f64 { 0.693147180559945309417232121458176568_f64 }
    #[inline(always)] static pure fn ln_10()            -> f64 { 2.30258509299404568401799145468436421_f64  }
}

pub impl float: IntConsts {
    #[inline(always)] static pure fn zero()             -> float { 0f }
    #[inline(always)] static pure fn one()              -> float { 1f }
}

pub impl float: FloatConsts {
    #[inline(always)] static pure fn two_pi()           -> float { 6.28318530717958647692528676655900576  }
    #[inline(always)] static pure fn pi()               -> float { 3.14159265358979323846264338327950288  }
    #[inline(always)] static pure fn pi_2()             -> float { 1.57079632679489661923132169163975144  }
    #[inline(always)] static pure fn pi_3()             -> float { 1.04719755119659774615421446109316763  }
    #[inline(always)] static pure fn pi_4()             -> float { 0.78539816339744830961566084581987572  }
    #[inline(always)] static pure fn pi_6()             -> float { 0.52359877559829887307710723054658381  }
    #[inline(always)] static pure fn pi_8()             -> float { 0.39269908169872415480783042290993786  }
    #[inline(always)] static pure fn frac_pi_2()        -> float { 1.57079632679489661923132169163975144  }
    #[inline(always)] static pure fn frac_pi_4()        -> float { 0.785398163397448309615660845819875721 }
    #[inline(always)] static pure fn frac_1_pi()        -> float { 0.318309886183790671537767526745028724 }
    #[inline(always)] static pure fn frac_2_pi()        -> float { 0.636619772367581343075535053490057448 }
    #[inline(always)] static pure fn frac_2_sqrtpi()    -> float { 1.12837916709551257389615890312154517  }
    #[inline(always)] static pure fn sqrt2()            -> float { 1.41421356237309504880168872420969808  }
    #[inline(always)] static pure fn frac_1_sqrt2()     -> float { 0.707106781186547524400844362104849039 }
    #[inline(always)] static pure fn e()                -> float { 2.71828182845904523536028747135266250  }
    #[inline(always)] static pure fn log2_e()           -> float { 1.44269504088896340735992468100189214  }
    #[inline(always)] static pure fn log10_e()          -> float { 0.434294481903251827651128918916605082 }
    #[inline(always)] static pure fn ln_2()             -> float { 0.693147180559945309417232121458176568 }
    #[inline(always)] static pure fn ln_10()            -> float { 2.30258509299404568401799145468436421  }
}