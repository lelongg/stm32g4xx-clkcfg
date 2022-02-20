use std::io::{self, Write};

use embedded_time::rate::Hertz;

/// Prescaler
#[derive(Clone, Copy)]
pub enum Prescaler {
    NotDivided,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
    Div512,
}

impl Prescaler {
    fn write_code(&self, to: &mut dyn Write) -> io::Result<()> {
        match self {
            Prescaler::NotDivided => write!(to, "Prescaler::NotDivided")?,
            Prescaler::Div2 => write!(to, "Prescaler::Div2")?,
            Prescaler::Div4 => write!(to, "Prescaler::Div4")?,
            Prescaler::Div8 => write!(to, "Prescaler::Div8")?,
            Prescaler::Div16 => write!(to, "Prescaler::Div16")?,
            Prescaler::Div32 => write!(to, "Prescaler::Div32")?,
            Prescaler::Div64 => write!(to, "Prescaler::Div64")?,
            Prescaler::Div128 => write!(to, "Prescaler::Div128")?,
            Prescaler::Div256 => write!(to, "Prescaler::Div256")?,
            Prescaler::Div512 => write!(to, "Prescaler::Div512")?,
        }
        Ok(())
    }
}

/// System clock mux source
#[derive(Clone, Copy)]
pub enum SysClockSrc {
    PLL,
    HSI,
    HSE(Hertz),
}

impl SysClockSrc {
    fn write_code(&self, to: &mut dyn Write) -> io::Result<()> {
        match self {
            SysClockSrc::PLL => write!(to, "SysClockSrc::PLL")?,
            SysClockSrc::HSI => write!(to, "SysClockSrc::HSI")?,
            SysClockSrc::HSE(freq) => write!(to, "SysClockSrc::HSE(Hertz({}))", freq)?,
        }
        Ok(())
    }
}

/// Microcontroller clock output source
pub enum MCOSrc {
    LSI,
    PLL,
    SysClk,
    HSI,
    HSE,
    LSE,
}

/// Low-speed clocks output source
pub enum LSCOSrc {
    LSI,
    LSE,
}

/// PLL clock input source
#[derive(Clone, Copy)]
pub enum PLLSrc {
    HSI,
    HSE(Hertz),
    HSEBypass(Hertz),
}

impl PLLSrc {
    fn write_code(&self, to: &mut dyn Write) -> io::Result<()> {
        match self {
            PLLSrc::HSI => write!(to, "PLLSrc::HSI")?,
            PLLSrc::HSE(freq) => write!(to, "PLLSrc::HSE(Hertz({}))", freq)?,
            PLLSrc::HSEBypass(freq) => write!(to, "PLLSrc::HSE_BYPASS(Hertz({}))", freq)?,
        }
        Ok(())
    }
}

/// PLL divider
pub type PLLDiv = u8;

/// PLL multiplier
pub type PLLMul = u8;

/// PLL config
#[derive(Clone, Copy)]
pub struct PllConfig {
    pub mux: PLLSrc,
    pub m: PLLDiv,
    pub n: PLLMul,
    pub r: PLLDiv,
    pub q: Option<PLLDiv>,
    pub p: Option<PLLDiv>,
}

impl PllConfig {
    fn write_code(&self, to: &mut dyn Write) -> io::Result<()> {
        write!(to, "ClockConfig {{\n")?;
        write!(to, "mux: ")?;
        self.mux.write_code(to)?;
        write!(to, ",\nm: {}", self.m)?;
        write!(to, ",\nn: {}", self.n)?;
        write!(to, ",\nr: {}", self.r)?;
        write!(to, ",\nq: {:?}", self.q)?;
        write!(to, ",\np: {:?}", self.p)?;
        write!(to, ",\n}}\n")?;
        Ok(())
    }
}

/// Clocks configutation
pub struct Config {
    pub(crate) sys_mux: SysClockSrc,
    pub(crate) pll_cfg: PllConfig,
    pub(crate) ahb_psc: Prescaler,
    pub(crate) apb_psc: Prescaler,
}

impl Config {
    pub fn write_code(&self, to: &mut dyn Write) -> io::Result<()> {
        write!(to, "Config::new()\n")?;
        write!(to, ".clock_src(")?;
        self.sys_mux.write_code(to)?;
        write!(to, ")\n.pll_cfg(")?;
        self.pll_cfg.write_code(to)?;
        write!(to, ")\n.ahb_psc(")?;
        self.ahb_psc.write_code(to)?;
        write!(to, ")\napb_psc(")?;
        self.apb_psc.write_code(to)?;
        write!(to, ")\n")?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct ClockRequirements {
    hse: Option<Hertz>,
    hse_bypass: bool,

    sysclk: Option<FreqReq>,
    hclk: Option<FreqReq>,
    pclk1: Option<FreqReq>,
    pclk2: Option<FreqReq>,
    usb_rng_clk: bool,
}

impl ClockRequirements {
    pub const fn new() -> Self {
        Self {
            hse: None,
            hse_bypass: false,
            sysclk: None,
            hclk: None,
            pclk1: None,
            pclk2: None,
            usb_rng_clk: false,
        }
    }

    pub fn hse<F: Into<Hertz>>(mut self, freq: F) -> Self {
        self.hse = Some(freq.into());
        self.hse_bypass = false;
        self
    }

    pub fn hse_bypass<F: Into<Hertz>>(mut self, freq: F) -> Self {
        self.hse = Some(freq.into());
        self.hse_bypass = true;
        self
    }

    pub fn sysclk<F: Into<FreqReq>>(mut self, freq: F) -> Self {
        self.sysclk = Some(freq.into());
        self
    }

    pub fn hclk<F: Into<FreqReq>>(mut self, freq: F) -> Self {
        self.hclk = Some(freq.into());
        self
    }

    pub fn pclk1<F: Into<FreqReq>>(mut self, freq: F) -> Self {
        self.pclk1 = Some(freq.into());
        self
    }

    pub fn pclk2<F: Into<FreqReq>>(mut self, freq: F) -> Self {
        self.pclk2 = Some(freq.into());
        self
    }

    pub fn usb_rng_clk(mut self, usb_rng_clk: bool) -> Self {
        self.usb_rng_clk = usb_rng_clk;
        self
    }

    pub fn resolve(
        mut self,
        ignore_hardware_caps: bool,
    ) -> Result<(Config, Frequencies), ClockError> {
        // We need to choose some sensible defaults if the user did not specify parts of the
        // clock tree.
        self.sensible_defaults();

        // Test whether the requirements and the HSE frequency make sense.
        if !ignore_hardware_caps {
            // HSE must be at most 48MHz.
            // TODO

            // sysclk, hclk, pclk1, and pclk2 must be at most 170MHz.
            // TODO
        }

        // For each possible clock configuration, we calculate the resulting frequencies, the
        // deviation from our requirements and whether the hardware actually supports the
        // frequency.
        self.search_config(|reqs, config, freqs| reqs.calculate_error(config, freqs))
    }

    pub fn search_config<
        F: Fn(&mut ClockRequirements, &Config, &Frequencies) -> Result<f64, ClockError>,
    >(
        &mut self,
        f: F,
    ) -> Result<(Config, Frequencies), ClockError> {
        // We simply perform a brute-force search over all configurations. This code can be
        // massively optimized.
        let mut best_config = None;
        let mut best_error = 0.0;

        // The PLL is supplied with HSE or HSI16.
        let pll_source = self.hse.unwrap_or(Hertz(16_000_000)).0 as f64;

        let mut freqs = Frequencies {
            pllp: 0.0,
            pllq: 0.0,
            pllr: 0.0,

            sysclk: 0.0,
            hclk: 0.0,
            pclk1: 0.0,
            pclk2: 0.0,
            usb_rng_clk: 0.0,
        };

        // We loop through all PLL configurations, filtering out those that result in illegal
        // intermediate frequencies.
        for pllm in 1..=16 {
            // The PLL input needs to be between 2.66 and 16 MHz.
            let pll_input = pll_source / pllm as f64;
            if pll_input < 2_660_000.0 || pll_input > 16_000_000.0 {
                continue;
            }
            for plln in 8..=127 {
                // The VCO output needs to be between 96 and 344 MHz.
                let vco = pll_input * plln as f64;
                if vco < 96_000_000.0 || vco > 344_000_000.0 {
                    continue;
                }

                // PLLP and PLLQ are optional. We always generate them and disable them later if
                // they are not required.
                for pllp in 2..=31 {
                    // The P output needs to be between 2.0645 and 170 MHz.
                    freqs.pllp = vco / pllp as f64;
                    if freqs.pllp < 2_064_500.0 || freqs.pllp > 170_000_000.0 {
                        continue;
                    }
                    for pllr in [2, 4, 6, 8] {
                        // The R output needs to be between 8 and 170 MHz.
                        freqs.pllr = vco / pllr as f64;
                        if freqs.pllr < 8_000_000.0 || freqs.pllr > 170_000_000.0 {
                            continue;
                        }
                        for pllq in [2, 4, 6, 8] {
                            // The Q output needs to be between 8 and 170 MHz.
                            freqs.pllq = vco / pllq as f64;
                            if freqs.pllq < 8_000_000.0 || freqs.pllq > 170_000_000.0 {
                                continue;
                            }
                            // TODO: HSI48?
                            freqs.usb_rng_clk = freqs.pllq;

                            let pll_cfg = PllConfig {
                                mux: if let Some(hse) = self.hse {
                                    if self.hse_bypass {
                                        PLLSrc::HSEBypass(hse)
                                    } else {
                                        PLLSrc::HSE(hse)
                                    }
                                } else {
                                    PLLSrc::HSI
                                },
                                m: pllm,
                                n: plln,
                                p: Some(pllp),
                                q: Some(pllq),
                                r: pllr,
                            };

                            // sysclk either comes from the PLL or directly from
                            // the selected clock source.
                            let hsi_or_hse = if let Some(hse) = self.hse {
                                (SysClockSrc::HSE(hse), hse.0 as f64)
                            } else {
                                (SysClockSrc::HSI, 16_000_000.0)
                            };
                            for (sys_mux, sysclk) in [hsi_or_hse, (SysClockSrc::PLL, freqs.pllr)] {
                                freqs.sysclk = sysclk;
                                for (ahb_psc, div) in [
                                    (Prescaler::NotDivided, 1.0),
                                    (Prescaler::Div2, 2.0),
                                    (Prescaler::Div4, 4.0),
                                    (Prescaler::Div8, 8.0),
                                    (Prescaler::Div16, 16.0),
                                    (Prescaler::Div32, 32.0),
                                    (Prescaler::Div64, 64.0),
                                    (Prescaler::Div128, 128.0),
                                    (Prescaler::Div256, 256.0),
                                    (Prescaler::Div512, 512.0),
                                ] {
                                    freqs.hclk = sysclk / div;
                                    for (apb_psc, div) in [
                                        (Prescaler::NotDivided, 1.0),
                                        (Prescaler::Div2, 2.0),
                                        (Prescaler::Div4, 4.0),
                                        (Prescaler::Div8, 8.0),
                                        (Prescaler::Div16, 16.0),
                                    ] {
                                        freqs.pclk1 = freqs.hclk / div;
                                        freqs.pclk2 = freqs.hclk / div;

                                        // Generate the config.
                                        let cfg = Config {
                                            sys_mux: sys_mux.clone(),
                                            pll_cfg,
                                            ahb_psc,
                                            apb_psc,
                                        };

                                        // Test the config, only retain the config with the lowest
                                        // error.
                                        match f(self, &cfg, &freqs) {
                                            Ok(error) => {
                                                if best_config.is_none() || best_error > error {
                                                    best_config = Some((cfg, freqs.clone()));
                                                    best_error = error;
                                                }
                                            }
                                            Err(_e) => {
                                                // Retain the error that is most specific.
                                                // TODO
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Disable PLLQ and PLLR if they are not required.
        // TODO

        match best_config {
            Some(cfg) => Ok(cfg),
            None => Err(ClockError::NoClockFound("no clock found".to_owned())),
        }
    }

    fn calculate_error(&self, _config: &Config, freqs: &Frequencies) -> Result<f64, ClockError> {
        // For now, we always try to generate the 48MHz USB frequency from the PLL. This frequency
        // needs to be precise.
        // TODO: Proper frequency range, use HSI48 if required.
        if self.usb_rng_clk {
            let target = 48_000_000.0;
            if freqs.usb_rng_clk < target - 1000.0 {
                return Err(ClockError::NoClockFound("USB/RNG clock too low".to_owned()));
            }
            if freqs.usb_rng_clk > target + 1000.0 {
                return Err(ClockError::NoClockFound(
                    "USB/RNG clock too high".to_owned(),
                ));
            }
        }

        // Test whether the frequencies are within the required bounds and calculate the deviation
        // from the requested frequency.
        let mut error = 0.0;
        for (req, freq, name) in [
            (self.sysclk.as_ref(), freqs.sysclk, "sysclk"),
            (self.hclk.as_ref(), freqs.sysclk, "hclk"),
            (self.pclk1.as_ref(), freqs.sysclk, "pclk1"),
            (self.pclk2.as_ref(), freqs.sysclk, "pclk2"),
        ] {
            if let Some(req) = req {
                if let Some(min) = req.min {
                    if freq < min.0 as f64 {
                        return Err(ClockError::NoClockFound(format!("{} too low", name)));
                    }
                }
                if let Some(max) = req.max {
                    if freq > max.0 as f64 {
                        return Err(ClockError::NoClockFound(format!("{} too high", name)));
                    }
                }
                if let Some(target) = req.freq {
                    let dev = (freq - target.0 as f64) / target.0 as f64;
                    error += dev * dev;
                }
            }
        }

        Ok(error)
    }

    fn sensible_defaults(&mut self) {
        // If neither sysclk, hclk, pclk1, or pclk2 are given, we set everything
        // to 48MHz.
        if self.sysclk.is_none()
            && self.hclk.is_none()
            && self.pclk1.is_none()
            && self.pclk2.is_none()
        {
            self.hclk = Some(Hertz(48_000_000).into());
            // Note that pclk* automatically follow because we try lower
            // prescalers first.
        }
    }
}

#[derive(Clone)]
pub struct Frequencies {
    pub pllp: f64,
    pub pllq: f64,
    pub pllr: f64,

    pub sysclk: f64,
    pub hclk: f64,
    pub pclk1: f64,
    pub pclk2: f64,
    pub usb_rng_clk: f64,
}

#[derive(Clone)]
pub struct FreqReq {
    freq: Option<Hertz>,
    min: Option<Hertz>,
    max: Option<Hertz>,
}

impl FreqReq {
    pub fn new(freq: Hertz) -> FreqReq {
        FreqReq::from(freq)
    }

    pub fn precise(freq: Hertz) -> FreqReq {
        FreqReq {
            freq: Some(freq),
            min: Some(freq),
            max: Some(freq),
        }
    }

    pub fn min(mut self, min: Hertz) -> FreqReq {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: Hertz) -> FreqReq {
        self.max = Some(max);
        self
    }
}

impl From<Hertz> for FreqReq {
    fn from(freq: Hertz) -> FreqReq {
        FreqReq {
            freq: Some(freq),
            min: None,
            max: None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClockError {
    #[error("requirements are outside of hardware capabilities: {0}")]
    BadRequirements(String),
    // TODO: Errors should have priorities, and in our search, we only return the error that has the highest priority
    // (i.e., was found deepest in the clock tree).
    #[error("requirements cannot be fulfilled: {0}")]
    NoClockFound(String),
    // TODO
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_defaults() {
        let reqs = ClockRequirements::new();
    }
}
