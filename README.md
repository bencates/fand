# Raspberry Pi Fan Daemon

I have a Zebra Virtue fan cooled Raspberry Pi 3 case, which is unfortunately
quite loud. This is a small daemon to control the fan speed based on the Pi's
internal temperature sensor.

In my setup the fan is on GPIO pin 18, a PWM pin that happens to be next to a
ground pin. The use of a PWM pin allows us to precisely control the fan speed.
