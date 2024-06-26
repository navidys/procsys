Supported Features

* `/sys/class/`
    * ✅ `dmi/id`
        * `bios_date`
        * `bios_release`
        * `bios_vendor`
        * `bios_version`
        * `board_asset_tag`
        * `board_name`
        * `board_serial`
        * `board_vendor`
        * `board_version`
        * `chassis_asset_tag`
        * `chassis_serial`
        * `chassis_type`
        * `chassis_vendor`
        * `product_family`
        * `product_name`
        * `product_serial`
        * `product_sku`
        * `product_uuid`
        * `sys_vendor`

    * ✅ `thermal/cooling_device<X>`
        * `type`
        * `max_state`
        * `cur_state`

    * ✅ `thermal/thermal_zone<X>`
        * `type`
        * `temp`
        * `policy`
        * `mode`
        * `passive`

    * ✅ `watchdog/<name>`
        * `bootstatus`
        * `options`
        * `fw_version`
        * `identity`
        * `nowayout`
        * `state`
        * `status`
        * `timeleft`
        * `timeout`
        * `min_timeout`
        * `max_timeout`
        * `pretimeout`
        * `pretimeout_governor`
        * `access_cs0`

* `/sys/devices/system/`
    * ✅ `clocksource/clocksource<X>`
        * `available_clocksource`
        * `current_clocksource`
