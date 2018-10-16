offregisters-lib
================

Concurrent task runner library, written in Rust.

This library is used notably by [offregisters-ctl](https://github.com/offregisters-ctl), but also by all the deployment binaries we
write, such as: `offregisters-postgres`; `offregisters-redis`; `offregisters-nginx`; `offregisters-python`; `offregisters-nodejs`; 
`offregisters-mesos`; &etc.

## Config example
```yaml
name: taiga
version: 0.0.1-alpha
depends:
- exec: offregister-postgres
   args:
   - --version 9.6.4
   - --debug --foo
   env:
      FOO: haz  # inner `env` takes precendence over outer `env`
- exec: offregister-python
env:
  FOO: bar
  CAN: HAZ
pipe: offmetric --server influxdb://influx.offscale.io
```

---

For more ideas we're considering—and to add your own—see: https://github.com/offscale/offscale-rfcs
