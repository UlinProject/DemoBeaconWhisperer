

document.getElementById('start').addEventListener('click', start);
document.getElementById('stop').addEventListener('click', stop);

function sleep(ms) {
	return new Promise(resolve => setTimeout(resolve, ms));
}

const bkey_device_uuid = [ // KEYO 16bytes
	0, 0, 5, 4, 
];

const current_device_uuid = [ // KEYO 16bytes
	//ALWAYS RUNDOM
	0, 0, 0, 0, 
];

function transform_arraytouuid(array) {
	return stringify(array);
}

var ble_adapter_info;
var a_beacons; // МАЯКИ
var i_a_beacon; // НОМЕР АКТИВНОГО МАЯКА
var c_full_i_a_beacon; // КОЛИЧЕСТВО_ПОЛНЫХ_АКТИВНЫХ_МАЯКОВ 4 3 2 1 (-->СКОЛЬКО РАЗ КРУТИТЬ ОДНИ И ТЕЖЕ МАЯКИ ПО КРУГУ? ->)
var end_beacons_fn; // ДЕЙСТВИЕ ПРИ НЕУДАЧНОМ ОКОНЧАНИИ КАРУСЕЛИ МАЯКОВ
var detect_minuts_reset; // ПРИВЯЗКА К УНИВЕРСАЛЬНЫМ МИНУТАМ (ЕСЛИ +5 МИНУТ ТО МАЯКИ ПРОТУХЛИ)
var fn_rebuild_a_beacons; // ДЕЙСТВИЕ ПРИ ПРОТУХШИХ МАЯКАХ (ОБЫЧНО ПЕРЕСОЗДАНИЕ ИХ)
var a_c_pos; // false 0..->, true 0..<-

async function send_hidata() {
	logToDom('Adapter ' + JSON.stringify(ble_adapter_info));
	
	a_c_pos = Math.random() < 0.5; // true | false
	detect_minuts_reset = null;
	fn_rebuild_a_beacons = async () => {
		let data = await make_protocol_bleconnect_to_me(null, null);
		if (data === null) {
			return null;
		}
		a_beacons = make_arraybeacons_rw_region(
			bkey_device_uuid, current_device_uuid,
			
			/*[
				0, 1, 2, 3, 4, 5, 6, 7, 
				8, 9, 10, 11, 12, 13, 14, 15,
				16, 17
			]*/
			data
		);
		return true;
	};
	if ((await fn_rebuild_a_beacons()) === null) {
		return null;
	}
	c_full_i_a_beacon = 5;
	if (!a_c_pos) {
		// false
		i_a_beacon = 0;
	}else {
		// true
		i_a_beacon = a_beacons.length-1;
	}
	await cordova.plugins.locationManager.isAdvertisingAvailable()
		.then(async function(isSupported) {
			if (isSupported) {
				logToDom("wait_next_beacons, START_NUM: " + i_a_beacon);
				
				end_beacons_fn = function (e) {
					logToDom("END. OK");
					
					set_statustxt("SEND_BEACON END");
					cordova.plugins.locationManager.stopAdvertising()
						.fail(function(e) { logToDom("64" + e); })
						.done();

					cordova.plugins.locationManager.disableBluetooth();
				};
				
				if (!a_c_pos) {
					// false
					if (i_a_beacon < a_beacons.length) {
						await wait_next_beacons();
					}else {
						if (end_beacons_fn !== null) {
							end_beacons_fn("82");
							end_beacons_fn = null;
						}
					}
				}else {
					// true
					if (i_a_beacon > 0) {
						await wait_next_beacons();
					}else {
						if (end_beacons_fn !== null) {
							end_beacons_fn("82");
							end_beacons_fn = null;
						}
					}
				}
			} else {
				set_statustxt("Advertising not supported");
				logToDom("Advertising not supported");
			}
		})
		.fail(function(e) { logToDom('53' + e); })
		.done();
}

async function wait_next_beacons() {
	logToDom("ACTIVE: " + i_a_beacon);
	
	let str_acposmove;
	if (!a_c_pos) {
		str_acposmove = '->';
	}else {
		str_acposmove = '<-';
	}
	
	set_statustxt('SEND_BEACON ACTIVE, len(' + a_beacons.length + '), (' + c_full_i_a_beacon  + '), a(' + i_a_beacon + ') ' + str_acposmove);

	logToDom("Advertising, beacon:" + JSON.stringify(a_beacons[i_a_beacon].beacon));
	logToDom("Advertising, data:" + JSON.stringify(a_beacons[i_a_beacon].data));
	
	logToDom("startAdvertising");
	await cordova.plugins.locationManager.startAdvertising(a_beacons[i_a_beacon].beacon)
		.done(async function(data) {
			logToDom('wait 1sec');
			await sleep(2000);
			await loop_next_beacon();
		})
		.fail(function(e) {
			logToDom('86' + e);
			cordova.plugins.locationManager.stopAdvertising()
				.fail(function(e) { logToDom('64' + e); })
				.done();
			
			
			if (end_beacons_fn !== null) {
				end_beacons_fn(e);
				end_beacons_fn = null;
			}
		}).done();
}

async function loop_next_beacon() {
	logToDom("stopAdvertising");
	cordova.plugins.locationManager.stopAdvertising()
		.fail(function(e) { logToDom("64" + e); })
		.done();
	
	if (detect_minuts_reset !== null) {
		let mi = make_universal_time().mi;
		if (detect_minuts_reset != mi) {
			rand_cuuid();
			detect_minuts_reset = null;
			c_full_i_a_beacon += 4;
			logToDom("TRIG REBUILD_M");
			
			if (fn_rebuild_a_beacons !== null) {
				if (await fn_rebuild_a_beacons() === null) {
					logToDom("TRIG REBUILD_M: ERR");
					if (end_beacons_fn !== null) {
						end_beacons_fn('#end err_rebuild_ibeacons');
						end_beacons_fn = null;
					}
					return;
				}
			}
			// TRIG REBUILD_M
			if (!a_c_pos) {
				// false
				i_a_beacon = 0;
			}else {
				// true
				i_a_beacon = a_beacons.length-1;
			}
			
		}
	}
	
	if (!a_c_pos) {
		// false 
		if (i_a_beacon < a_beacons.length-1) {
			i_a_beacon += 1;
		}else {
			c_full_i_a_beacon -= 1;
			i_a_beacon = 0;
			
			if (c_full_i_a_beacon <= 0) {
				if (end_beacons_fn !== null) {
					end_beacons_fn('#end c_full_i_beacon');
					end_beacons_fn = null;
				}
				return;
			}
		}
	}else {
		// true
		if (i_a_beacon > 0) {
			i_a_beacon -= 1;
		}else {
			c_full_i_a_beacon -= 1;
			i_a_beacon = a_beacons.length-1;
			
			if (c_full_i_a_beacon <= 0) {
				if (end_beacons_fn !== null) {
					end_beacons_fn('#end c_full_i_beacon');
					end_beacons_fn = null;
				}
				return;
			}
		}
	}
	
	
	return await wait_next_beacons();
}

function rand_cuuid() {
	let i = 0;
	while (i < current_device_uuid.length) {
		current_device_uuid[i] = Math.floor(256 * Math.random());
		i += 1;
	}
}

async function start() {
	cordova.plugins.locationManager.disableBluetooth();
	rand_cuuid();
	await sleep(1000);
	cordova.plugins.locationManager.enableBluetooth();
	await sleep(1000);
	
	cordova.plugins.locationManager.isBluetoothEnabled()
		.then(async function(isEnabled){
			//logToDom("isEnabled: " + isEnabled);
			if (!isEnabled) {
				set_statustxt('INVALID EN_BLUETOOTH; ERR');
				return;
			}
			set_statustxt('BLUETOOTH AUTOEN; OK');
			cordova.plugins.locationManager.requestWhenInUseAuthorization();

			cordova.plugins.locationManager.getAuthorizationStatus()
				.then(async function(authorized) {
					await networking.bluetooth.getAdapterState(async function (adapterInfo) {
						// The adapterInfo object has the following properties:
						// address: String --> The address of the adapter, in the format 'XX:XX:XX:XX:XX:XX'.
						// name: String --> The human-readable name of the adapter.
						// enabled: Boolean --> Indicates whether or not the adapter is enabled.
						// discovering: Boolean --> Indicates whether or not the adapter is currently discovering.
						// discoverable: Boolean --> Indicates whether or not the adapter is currently discoverable.
						
						ble_adapter_info = adapterInfo;
						await send_hidata();
					}, function (errorMessage) {
						logToDom('200' + errorMessage);
					});
				})
				.fail(function(e) { logToDom('176' + e); })
				.done();
		})
		.fail(function(e) { logToDom('182' + e); })
		.done();
}

function stop() {
	
}

