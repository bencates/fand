extern crate sysfs_pwm;

use self::sysfs_pwm::{Pwm, Result};

const GPIO_CHIP: u32 = 0;
const GPIO_PIN: u32 = 18;

pub struct Fan(Pwm);

impl Drop for Fan {
    fn drop(&mut self) {
        self.0.unexport().unwrap();
    }
}

impl Fan {
    pub fn new() -> Result<Fan> {
        let pwm = Pwm::new(GPIO_CHIP, GPIO_PIN)?;
        pwm.export()?;

        Ok(Fan(pwm))
    }

    pub fn set_speed(&self, fan_speed: f32) -> Result<()> {
        let period_ns: u32 = self.0.get_period_ns()?;
        let duty_cycle_ns = (fan_speed * period_ns as f32) as u32;

        self.0.set_duty_cycle_ns(duty_cycle_ns)?;

        Ok(())
    }
}
