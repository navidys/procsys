Supported Features
* ✅ `/proc/<pid>`
    * cgroup
    * cmdline
    * comm
    * cwd
    * environ
    * exe
    * io
    * limits
    * root
    * ns
    * net/
        * netstat

* ✅ `/proc/buddyinfo`

* ✅ `/proc/cmdline`

* ✅ `/proc/cpuinfo`

* ✅ `/proc/loadavg`

* ✅ `/proc/meminfo`

* ✅ `/proc/net/dev`

* ✅ `/proc/net/protocols`

* ✅ `/proc/sys/kernel/random`
    * entropy_avail
    * poolsize
    * urandom_min_reseed_secs
    * write_wakeup_threshold
    * read_wakeup_threshold

* ✅ `/proc/swaps`

* ✅ `/sys/class/dmi/id`
    * bios_date
    * bios_release
    * bios_vendor
    * bios_version
    * board_asset_tag
    * board_name
    * board_serial
    * board_vendor
    * board_version
    * chassis_asset_tag
    * chassis_serial
    * chassis_type
    * chassis_vendor
    * product_family
    * product_name
    * product_serial
    * product_sku
    * product_uuid
    * sys_vendor

* ✅ `/sys/thermal/cooling_device<X>`
    * type
    * max_state
    * cur_state

* ✅ `/sys/thermal/thermal_zone<X>`
    * type
    * temp
    * policy
    * mode
    * passive

* ✅ `/sys/watchdog/<name>`
    * bootstatus
    * options
    * fw_version
    * identity
    * nowayout
    * state
    * status
    * timeleft
    * timeout
    * min_timeout
    * max_timeout
    * pretimeout
    * pretimeout_governor
    * access_cs0

* ✅ `/sys/devices/system/clocksource/clocksource<X>`
    * available_clocksource
    * current_clocksource
