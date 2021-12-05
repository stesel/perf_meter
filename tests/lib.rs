#[cfg(test)]
mod tests {
    extern crate perf_meter;

    use std::time::{Duration, Instant};
    use perf_meter::PerfMeter;

    #[test]
    fn should_create_perf_meter_instance() {
        let secs = 1_u64;
        let pm = PerfMeter::new(secs);
        assert_eq!(pm.ops_count, 0);
        assert_eq!(pm.interval.as_secs(), secs);
    }

    #[test]
    fn should_update_counter_on_tick() {
        let secs = 1_u64;
        let mut pm = PerfMeter::new(secs);
        pm.tick();
        assert_eq!(pm.ops_count, 1);
        pm.tick();
        assert_eq!(pm.ops_count, 2);
    }

    #[test]
    fn should_reset_ops_count_after_interval() {
        let secs = 1_u64;
        let mut pm = PerfMeter::new(secs);

        while (Instant::now() - pm.start) < pm.interval {
            pm.tick();
        }
        pm.tick();
        assert_eq!(pm.ops_count, 0);
    }
}
