use std::ffi::CStr;
use std::mem::size_of;
use std::mem::size_of_val;



use esp32_hal::efuse::Efuse;
/*use esp_idf_bindgen::BTDM_CTRL_AUTO_LATENCY_EFF;
use esp_idf_bindgen::BTDM_CTRL_HLI;
use esp_idf_bindgen::BTDM_CTRL_LEGACY_AUTH_VENDOR_EVT_EFF;
use esp_idf_bindgen::BT_HCI_UART_BAUDRATE_DEFAULT;
use esp_idf_bindgen::BT_HCI_UART_NO_DEFAULT;
use esp_idf_bindgen::CONFIG_BTDM_CONTROLLER_BLE_MAX_CONN_EFF;
use esp_idf_bindgen::CONFIG_BTDM_CONTROLLER_BR_EDR_MAX_ACL_CONN_EFF;
use esp_idf_bindgen::esp_bt_mode_t::ESP_BT_MODE_BTDM;
use esp_idf_bindgen::CONTROLLER_ADV_LOST_DEBUG_BIT;
use esp_idf_bindgen::ESP_BD_ADDR_LEN;
use esp_idf_bindgen::ESP_BT_CONTROLLER_CONFIG_MAGIC_VAL;
use esp_idf_bindgen::ESP_OK;
use esp_idf_bindgen::esp_bt_mode_t::ESP_BT_MODE_CLASSIC_BT;
use esp_idf_bindgen::ESP_TASK_BT_CONTROLLER_PRIO;
use esp_idf_bindgen::ESP_TASK_BT_CONTROLLER_STACK;
use esp_idf_bindgen::ESP_UUID_LEN_128;
use esp_idf_bindgen::MESH_DUPLICATE_SCAN_CACHE_SIZE;
use esp_idf_bindgen::NORMAL_SCAN_DUPLICATE_CACHE_SIZE;
use esp_idf_bindgen::SCAN_DUPLICATE_MODE;
use esp_idf_bindgen::SCAN_DUPLICATE_TYPE_VALUE;
use esp_idf_bindgen::SCAN_SEND_ADV_RESERVED_SIZE;
use esp_idf_bindgen::esp_bt_status_t;
use esp_idf_bindgen::esp_ble_addr_type_t;
use esp_idf_bindgen::esp_ble_gap_cb_param_t;
use esp_idf_bindgen::esp_ble_gap_register_callback;
use esp_idf_bindgen::esp_ble_gap_set_scan_params;
use esp_idf_bindgen::esp_ble_gap_start_scanning;
use esp_idf_bindgen::esp_ble_scan_duplicate_t;
use esp_idf_bindgen::esp_ble_scan_filter_t;
use esp_idf_bindgen::esp_bluedroid_enable;
use esp_idf_bindgen::esp_bluedroid_init;
use esp_idf_bindgen::esp_bt_controller_enable;
use esp_idf_bindgen::esp_bt_mode_t::ESP_BT_MODE_BLE;
use esp_idf_bindgen::esp_ble_scan_params_t;
use esp_idf_bindgen::esp_ble_scan_type_t;
use esp_idf_bindgen::esp_bt_controller_config_t;
use esp_idf_bindgen::esp_bt_controller_init;
use esp_idf_bindgen::esp_bt_controller_mem_release;
use esp_idf_bindgen::esp_bt_mode_t;
use esp_idf_bindgen::esp_err_t;
use esp_idf_bindgen::esp_err_to_name;
use esp_idf_bindgen::esp_gap_ble_cb_event_t;
use esp_idf_bindgen::esp_gap_search_evt_t;*/
use esp_idf_hal::mutex::Mutex;
use esp_idf_sys as _; 
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys::BTDM_CTRL_AUTO_LATENCY_EFF;
use esp_idf_sys::BTDM_CTRL_LEGACY_AUTH_VENDOR_EVT_EFF;
use esp_idf_sys::BT_HCI_UART_BAUDRATE_DEFAULT;
use esp_idf_sys::BT_HCI_UART_NO_DEFAULT;
use esp_idf_sys::CONFIG_BTDM_BLE_SLEEP_CLOCK_ACCURACY_INDEX_EFF;
use esp_idf_sys::CONFIG_BTDM_CTRL_BLE_MAX_CONN_EFF;
use esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_MAX_ACL_CONN_EFF;
use esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_MAX_SYNC_CONN_EFF;
use esp_idf_sys::CONFIG_BTDM_CTRL_BR_EDR_SCO_DATA_PATH_EFF;
use esp_idf_sys::CONFIG_BTDM_CTRL_HLI;
use esp_idf_sys::CONFIG_BTDM_CTRL_MODE_BLE_ONLY;
//use esp_idf_sys::CONFIG_BTDM_CTRL_MODE_BTDM;
use esp_idf_sys::CONFIG_BTDM_CTRL_PCM_POLAR_EFF;
use esp_idf_sys::CONFIG_BTDM_CTRL_PCM_ROLE_EFF;
use esp_idf_sys::CONTROLLER_ADV_LOST_DEBUG_BIT;
use esp_idf_sys::ESP_BD_ADDR_LEN;
use esp_idf_sys::ESP_TASK_BT_CONTROLLER_STACK;
use esp_idf_sys::ESP_UUID_LEN_128;
use esp_idf_sys::EspError;
use esp_idf_hal::delay::FreeRtos;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_sys::MESH_DUPLICATE_SCAN_CACHE_SIZE;
use esp_idf_sys::NORMAL_SCAN_DUPLICATE_CACHE_SIZE;
use esp_idf_sys::SCAN_DUPLICATE_MODE;
use esp_idf_sys::SCAN_DUPLICATE_TYPE_VALUE;
use esp_idf_sys::SCAN_SEND_ADV_RESERVED_SIZE;
use esp_idf_sys::c_types::c_void;
use esp_idf_sys::esp_ble_addr_type_t;
use esp_idf_sys::esp_ble_addr_type_t_BLE_ADDR_TYPE_PUBLIC;
use esp_idf_sys::esp_ble_gap_cb_param_t;
use esp_idf_sys::esp_ble_gap_register_callback;
use esp_idf_sys::esp_ble_gap_set_scan_params;
use esp_idf_sys::esp_ble_gap_start_scanning;
use esp_idf_sys::esp_ble_scan_duplicate_t;
use esp_idf_sys::esp_ble_scan_duplicate_t_BLE_SCAN_DUPLICATE_DISABLE;
use esp_idf_sys::esp_ble_scan_filter_t;
use esp_idf_sys::BTDM_CTRL_HLI;
use esp_idf_sys::ESP_TASK_BT_CONTROLLER_PRIO;
use esp_idf_sys::esp_ble_scan_filter_t_BLE_SCAN_FILTER_ALLOW_ALL;
use esp_idf_sys::esp_ble_scan_params_t;
use esp_idf_sys::esp_ble_scan_type_t;
use esp_idf_sys::esp_ble_scan_type_t_BLE_SCAN_TYPE_ACTIVE;
use esp_idf_sys::esp_bluedroid_enable;
use esp_idf_sys::esp_bluedroid_init;
use esp_idf_sys::esp_bt_controller_config_t;
use esp_idf_sys::esp_bt_controller_enable;
use esp_idf_sys::esp_bt_controller_init;
use esp_idf_sys::esp_bt_controller_mem_release;
use esp_idf_sys::esp_bt_mode_t_ESP_BT_MODE_BLE;
use esp_idf_sys::esp_bt_mode_t_ESP_BT_MODE_BTDM;
use esp_idf_sys::esp_bt_mode_t_ESP_BT_MODE_CLASSIC_BT;
use esp_idf_sys::esp_bt_status_t;
use esp_idf_sys::esp_bt_status_t_ESP_BT_STATUS_SUCCESS;
use esp_idf_sys::esp_gap_ble_cb_event_t;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_RAW_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_START_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_STOP_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_AUTH_CMPL_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_CHANNEL_SELETE_ALGORITHM_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_CLEAR_BOND_DEV_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EVT_MAX;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_DATA_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_CLEAR_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_PARAMS_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_RAND_ADDR_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_REMOVE_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_START_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_STOP_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_SCAN_RSP_DATA_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_GET_BOND_DEV_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_KEY_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_LOCAL_ER_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_LOCAL_IR_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_NC_REQ_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_OOB_REQ_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PASSKEY_NOTIF_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PASSKEY_REQ_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_ADD_DEV_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_CREATE_SYNC_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_DATA_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_REPORT_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SET_PARAMS_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_START_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_STOP_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_CANCEL_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_ESTAB_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_LOST_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_TERMINATE_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_READ_PHY_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_READ_RSSI_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_REMOVE_BOND_DEV_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_PARAM_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RESULT_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_RAW_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_SET_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_START_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_STOP_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SEC_REQ_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_CHANNELS_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_LOCAL_PRIVACY_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_PKT_LENGTH_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_PREFERED_DEFAULT_PHY_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_PREFERED_PHY_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_STATIC_RAND_ADDR_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_CONN_PARAMS_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_DUPLICATE_EXCEPTIONAL_LIST_COMPLETE_EVT;
use esp_idf_sys::esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_WHITELIST_COMPLETE_EVT;
use esp_idf_sys::esp_gap_search_evt_t_ESP_GAP_SEARCH_DISC_BLE_RES_EVT;
use esp_idf_sys::esp_gap_search_evt_t_ESP_GAP_SEARCH_DISC_CMPL_EVT;
use esp_idf_sys::esp_gap_search_evt_t_ESP_GAP_SEARCH_DISC_RES_EVT;
use esp_idf_sys::esp_gap_search_evt_t_ESP_GAP_SEARCH_DI_DISC_CMPL_EVT;
use esp_idf_sys::esp_gap_search_evt_t_ESP_GAP_SEARCH_INQ_CMPL_EVT;
use esp_idf_sys::esp_gap_search_evt_t_ESP_GAP_SEARCH_INQ_DISCARD_NUM_EVT;
use esp_idf_sys::esp_gap_search_evt_t_ESP_GAP_SEARCH_INQ_RES_EVT;
use esp_idf_sys::*;
use esp_idf_sys::ets_run;
use esp_idf_sys::memcmp;
use esp_idf_sys::nvs_flash_init;
use esp_idf_sys::vTaskDelete;
use esp_idf_sys::ESP_BT_CONTROLLER_CONFIG_MAGIC_VAL;
use once_cell::sync::OnceCell;
/*
use core::ffi::c_void;
use core::ffi::CStr;
use std::collections::HashMap;
use esp_idf_sys::xTaskCreatePinnedToCore;
use esp_idf_sys::ets_run;
use esp_idf_sys::esp_task_wdt_add;
use esp_idf_sys::vTaskDelete;
use esp_idf_hal::gpio::Gpio2;
use esp_idf_hal::gpio::Unknown;
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::cpu::core;

use std::fs::File;*/
use esp_idf_sys::esp_task_wdt_reset;

/*pub mod doc_std {
	pub use std::*;
}*/

static PERIPHERALS: OnceCell<Mutex<Peripherals>> = OnceCell::new();

// IBEACON_RECEIVER
static mut ble_scan_params: esp_ble_scan_params_t = esp_ble_scan_params_t {
	scan_type: esp_ble_scan_type_t_BLE_SCAN_TYPE_ACTIVE,
	own_addr_type: esp_ble_addr_type_t_BLE_ADDR_TYPE_PUBLIC,
	scan_filter_policy: esp_ble_scan_filter_t_BLE_SCAN_FILTER_ALLOW_ALL,
	scan_interval: 0x0010,
	scan_window: 0x0010,
	scan_duplicate: esp_ble_scan_duplicate_t_BLE_SCAN_DUPLICATE_DISABLE
};

pub fn BT_CONTROLLER_INIT_CONFIG_DEFAULT() -> esp_bt_controller_config_t {
	//Default::default()
	/*
	#define BT_CONTROLLER_INIT_CONFIG_DEFAULT() {                              \
	    .controller_task_stack_size = ESP_TASK_BT_CONTROLLER_STACK,            \
	    .controller_task_prio = ESP_TASK_BT_CONTROLLER_PRIO,                   \
	    .hci_uart_no = BT_HCI_UART_NO_DEFAULT,                                 \
	    .hci_uart_baudrate = BT_HCI_UART_BAUDRATE_DEFAULT,                     \
	    .scan_duplicate_mode = SCAN_DUPLICATE_MODE,                            \
	    .scan_duplicate_type = SCAN_DUPLICATE_TYPE_VALUE,                      \
	    .normal_adv_size = NORMAL_SCAN_DUPLICATE_CACHE_SIZE,                   \
	    .mesh_adv_size = MESH_DUPLICATE_SCAN_CACHE_SIZE,                       \
	    .send_adv_reserved_size = SCAN_SEND_ADV_RESERVED_SIZE,                 \
	    .controller_debug_flag = CONTROLLER_ADV_LOST_DEBUG_BIT,                \
	    .mode = BTDM_CONTROLLER_MODE_EFF,                                      \
	    .ble_max_conn = CONFIG_BTDM_CTRL_BLE_MAX_CONN_EFF,                     \
	    .bt_max_acl_conn = CONFIG_BTDM_CTRL_BR_EDR_MAX_ACL_CONN_EFF,           \
	    .bt_sco_datapath = CONFIG_BTDM_CTRL_BR_EDR_SCO_DATA_PATH_EFF,          \
	    .auto_latency = BTDM_CTRL_AUTO_LATENCY_EFF,                            \
	    .bt_legacy_auth_vs_evt = BTDM_CTRL_LEGACY_AUTH_VENDOR_EVT_EFF,         \
	    .bt_max_sync_conn = CONFIG_BTDM_CTRL_BR_EDR_MAX_SYNC_CONN_EFF,         \
	    .ble_sca = CONFIG_BTDM_BLE_SLEEP_CLOCK_ACCURACY_INDEX_EFF,             \
	    .pcm_role = CONFIG_BTDM_CTRL_PCM_ROLE_EFF,                             \
	    .pcm_polar = CONFIG_BTDM_CTRL_PCM_POLAR_EFF,                           \
	    .hli = BTDM_CTRL_HLI,                                                  \
	    .magic = ESP_BT_CONTROLLER_CONFIG_MAGIC_VAL,                           \
	};
	*/
	
	/*
		#if defined(CONFIG_BTDM_CTRL_MODE_BLE_ONLY)
		#define BTDM_CONTROLLER_MODE_EFF                    ESP_BT_MODE_BLE
		#elif defined(CONFIG_BTDM_CTRL_MODE_BR_EDR_ONLY)
		#define BTDM_CONTROLLER_MODE_EFF                    ESP_BT_MODE_CLASSIC_BT
		#else
		#define BTDM_CONTROLLER_MODE_EFF                    ESP_BT_MODE_BTDM
		#endif
	*/
	let BTDM_CONTROLLER_MODE_EFF;
	if CONFIG_BTDM_CTRL_MODE_BLE_ONLY == 1 {
		BTDM_CONTROLLER_MODE_EFF = esp_bt_mode_t_ESP_BT_MODE_BLE;
	}/*else*/
	
	//if CONFIG_BTDM_CTRL_MODE_BR_EDR_ONLY == 1 {
		//BTDM_CONTROLLER_MODE_EFF = CONFIG_BTDM_CTRL_MODE_BTDM;
	/*}else {
		BTDM_CONTROLLER_MODE_EFF = esp_bt_mode_t_ESP_BT_MODE_BTDM;
	}*/
	else {
		BTDM_CONTROLLER_MODE_EFF = esp_bt_mode_t_ESP_BT_MODE_BTDM;
	}
	
	
	esp_bt_controller_config_t {
		controller_task_stack_size: ESP_TASK_BT_CONTROLLER_STACK as _,
		controller_task_prio: ESP_TASK_BT_CONTROLLER_PRIO as _,
		hci_uart_no: BT_HCI_UART_NO_DEFAULT as _,
		hci_uart_baudrate: BT_HCI_UART_BAUDRATE_DEFAULT as _,
		scan_duplicate_mode: SCAN_DUPLICATE_MODE as _,
		scan_duplicate_type: SCAN_DUPLICATE_TYPE_VALUE as _,
		normal_adv_size: NORMAL_SCAN_DUPLICATE_CACHE_SIZE as _,
		mesh_adv_size: MESH_DUPLICATE_SCAN_CACHE_SIZE as _,
		send_adv_reserved_size: SCAN_SEND_ADV_RESERVED_SIZE as _,
		controller_debug_flag: CONTROLLER_ADV_LOST_DEBUG_BIT as _,
		mode: BTDM_CONTROLLER_MODE_EFF as _, 
		ble_max_conn: CONFIG_BTDM_CTRL_BLE_MAX_CONN_EFF as _,
		bt_max_acl_conn: CONFIG_BTDM_CTRL_BR_EDR_MAX_ACL_CONN_EFF as _,
		bt_sco_datapath: CONFIG_BTDM_CTRL_BR_EDR_SCO_DATA_PATH_EFF as _,
		auto_latency: BTDM_CTRL_AUTO_LATENCY_EFF == 1 as _,
		bt_legacy_auth_vs_evt: BTDM_CTRL_LEGACY_AUTH_VENDOR_EVT_EFF == 1 as _,
		bt_max_sync_conn: CONFIG_BTDM_CTRL_BR_EDR_MAX_SYNC_CONN_EFF as _,
		ble_sca: CONFIG_BTDM_BLE_SLEEP_CLOCK_ACCURACY_INDEX_EFF as _,
		pcm_role: CONFIG_BTDM_CTRL_PCM_ROLE_EFF as _,
		pcm_polar: CONFIG_BTDM_CTRL_PCM_POLAR_EFF as _,
		hli: BTDM_CTRL_HLI == 1 as _,
		magic: ESP_BT_CONTROLLER_CONFIG_MAGIC_VAL as _,
	}
}

unsafe fn ble_ibeacon_init() -> Result<(), EspError> {
	esp_error!(esp_bluedroid_init());
	esp_error!(esp_bluedroid_enable());
	ble_ibeacon_appRegister()?;
	
	Ok(())
}

unsafe fn ble_ibeacon_appRegister() -> Result<(), EspError> {
	println!("register callback");

	//register the scan callback function to the gap module
	esp_error!(unsafe { esp_ble_gap_register_callback(Some(esp_gap_cb)) });
	
	Ok(())
}

#[allow(unaligned_references)]
#[derive(Debug)]
#[repr(packed)]
struct esp_ble_ibeacon_head_t {
	flags: [u8; 3],
	length: u8,
	r#type: u8,
	company_id: u16,
	beacon_type: u16,
}


#[allow(unaligned_references)]
#[derive(Debug)]
#[repr(packed)]
struct esp_ble_ibeacon_vendor_t {
	proximity_uuid: [u8; 16],
	major: u16,
	minor: u16,
	measured_power: i8,
}

#[derive(Debug)]
#[repr(packed)]
struct esp_ble_ibeacon_t {
	ibeacon_head: esp_ble_ibeacon_head_t,
	ibeacon_vendor: esp_ble_ibeacon_vendor_t,
}

/* Constant part of iBeacon data */
static ibeacon_common_head: esp_ble_ibeacon_head_t = esp_ble_ibeacon_head_t {
	flags: [0x02, 0x01, 0x06],
	length: 0x1A,
	r#type: 0xFF,
	company_id: 0x004C,
	beacon_type: 0x1502
};

fn esp_ble_is_ibeacon_packet(adv_data: *const c_void, adv_data_len: u8) -> bool {
	let mut result = false;
 
	if (!adv_data.is_null()) && (adv_data_len == 0x1E) {
		unsafe {
			if !memcmp(adv_data, &ibeacon_common_head as *const _ as *const c_void, size_of_val(&ibeacon_common_head) as _) == 0 {
				result = true;
			}
		}
	}
 
	return result;
 }

unsafe extern "C" fn esp_gap_cb(event: esp_gap_ble_cb_event_t, param: *mut esp_ble_gap_cb_param_t) {
	println!("event: {:?}", event);
	match event {
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_SET_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_SET_COMPLETE_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_SET_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_SET_COMPLETE_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_PARAM_SET_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_PARAM_SET_COMPLETE_EVT");
			// небыло
			
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RESULT_EVT => {
			println!(">>esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RESULT_EVT");
			let scan_result = param as *mut esp_ble_gap_cb_param_t;
			
			println!("{:?}", (*scan_result).scan_rst.search_evt);
			match (*scan_result).scan_rst.search_evt {
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_INQ_RES_EVT => {
					println!("377 {:?}", &(*scan_result).scan_rst.ble_adv);
					println!("378 {:?}", (*scan_result).scan_rst.adv_data_len);
					if esp_ble_is_ibeacon_packet(&(*scan_result).scan_rst.ble_adv as *const _ as *const c_void, (*scan_result).scan_rst.adv_data_len) {
						let ibeacon_data = (&(*scan_result).scan_rst.ble_adv) as *const _ as *mut esp_ble_ibeacon_t;
						
						println!("IBEACON_DEMO: Device address: {:?} {:?}", (*scan_result).scan_rst.bda, ESP_BD_ADDR_LEN );
						println!("IBEACON_DEMO: Proximity UUID: {:?} {:?}", (*ibeacon_data).ibeacon_vendor.proximity_uuid, ESP_UUID_LEN_128);
						
						println!("beacon_data: {:?}", ibeacon_data);
					}
				},
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_INQ_CMPL_EVT => {},
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_DISC_RES_EVT => {},
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_DISC_BLE_RES_EVT => {},
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_DISC_CMPL_EVT => {},
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_DI_DISC_CMPL_EVT => {},
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_SEARCH_CANCEL_CMPL_EVT => {},
				a if a == esp_gap_search_evt_t_ESP_GAP_SEARCH_INQ_DISCARD_NUM_EVT => {},
				_ => {},
			}
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_RAW_SET_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_DATA_RAW_SET_COMPLETE_EVT");
			// RECV
			//the unit of the duration is second, 0 means scan permanently
			let duration = 0u32;
			esp_ble_gap_start_scanning(duration);
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_RAW_SET_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_RSP_DATA_RAW_SET_COMPLETE_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_START_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_START_COMPLETE_EVT");
			if (*param).adv_start_cmpl.status != esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
				println!("Adv start failed: {:?}", (*param).scan_start_cmpl.status);
			}
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_START_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_START_COMPLETE_EVT");
			if (*param).scan_start_cmpl.status != esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
				println!("Scan start failed: {:?}", (*param).scan_start_cmpl.status);
			}else {
				println!("Scan start.");
			}
		},
		
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_AUTH_CMPL_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_AUTH_CMPL_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_KEY_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_KEY_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SEC_REQ_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_SEC_REQ_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PASSKEY_NOTIF_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_PASSKEY_NOTIF_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PASSKEY_REQ_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_PASSKEY_REQ_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_OOB_REQ_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_OOB_REQ_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_LOCAL_IR_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_LOCAL_IR_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_LOCAL_ER_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_LOCAL_ER_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_NC_REQ_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_NC_REQ_EVT");
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_STOP_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_STOP_COMPLETE_EVT");
			
			if (*param).adv_stop_cmpl.status != esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
				println!("Adv stop failed: {:?}", (*param).adv_stop_cmpl.status);
			} else {
				println!("Stop adv successfully");
			}
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_STOP_COMPLETE_EVT => {
			println!("esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_STOP_COMPLETE_EVT");
			
			if (*param).scan_stop_cmpl.status != esp_bt_status_t_ESP_BT_STATUS_SUCCESS {
				println!("Scan stop failed: {:?}", ((*param).scan_stop_cmpl.status));
			} else {
				println!("Stop scan successfully");
			}
		},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_STATIC_RAND_ADDR_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_CONN_PARAMS_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_PKT_LENGTH_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_LOCAL_PRIVACY_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_REMOVE_BOND_DEV_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_CLEAR_BOND_DEV_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_GET_BOND_DEV_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_READ_RSSI_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_WHITELIST_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_UPDATE_DUPLICATE_EXCEPTIONAL_LIST_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_CHANNELS_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_READ_PHY_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_PREFERED_DEFAULT_PHY_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_PREFERED_PHY_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_RAND_ADDR_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_PARAMS_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_DATA_SET_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_SCAN_RSP_DATA_SET_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_START_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_STOP_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_REMOVE_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_SET_CLEAR_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SET_PARAMS_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_DATA_SET_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_START_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_STOP_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_CREATE_SYNC_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_CANCEL_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_TERMINATE_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_ADD_DEV_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_REMOVE_DEV_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_CLEAR_DEV_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SET_EXT_SCAN_PARAMS_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_SCAN_START_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_SCAN_STOP_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PREFER_EXT_CONN_PARAMS_SET_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PHY_UPDATE_COMPLETE_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EXT_ADV_REPORT_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_TIMEOUT_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_ADV_TERMINATED_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_SCAN_REQ_RECEIVED_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_CHANNEL_SELETE_ALGORITHM_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_REPORT_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_LOST_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_SYNC_ESTAB_EVT => {},
		event if event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_EVT_MAX => {},
		
		_ => {
			println!("unkevent: {:?}", event);
		},
	}
}

#[macro_export]
macro_rules! esp_error {
	[$expr:expr] => {
		if let Some(e) = EspError::from($expr) {
			println!("err, line: {:?}, err: {:?}", line!(), e);
			return Err(e);
		}
	};
}

fn main() -> Result<(), EspError> {
	esp_idf_sys::link_patches();
	
	if !Efuse::is_bluetooth_enabled() {
		let mut freertos = FreeRtos;
		loop {
			unsafe {
				esp_task_wdt_reset();
			}
			println!("Invalid efuse, exp bluetooth enable");
			freertos.delay_ms(1000u16);
		}
	}
	println!("mac: {:?}", Efuse::get_mac_address());
	/*if !Efuse::get_core_count() < 2 {
		let mut freertos = FreeRtos;
		loop {
			unsafe {
				esp_task_wdt_reset();
			}
			println!("Invalid efuse, exp two core kernel");
			freertos.delay_ms(1000u16);
		}
	}*/
	
	let peripherals = unsafe { Peripherals::new() };
	if let Err(_e) = PERIPHERALS.set(Mutex::new(peripherals)) {
		return Err(EspError::from(1).unwrap());
	}
	
	esp_error!(unsafe { nvs_flash_init() });
	esp_error!(unsafe { esp_bt_controller_mem_release(esp_bt_mode_t_ESP_BT_MODE_CLASSIC_BT) });
	
	let mut bt_cfg: esp_bt_controller_config_t = BT_CONTROLLER_INIT_CONFIG_DEFAULT();
	dbg!(&bt_cfg);
	esp_error!(unsafe { esp_bt_controller_init(&mut bt_cfg as _) });
	esp_error!(unsafe { esp_bt_controller_enable(esp_bt_mode_t_ESP_BT_MODE_BLE) });
	
	unsafe {
		ble_ibeacon_init()?;
	}
	
	// RECV
	esp_error!(unsafe { esp_ble_gap_set_scan_params(&mut ble_scan_params as _) });
	
	
	let duration = 0u32;
	unsafe {
		esp_ble_gap_start_scanning(duration);
	}
	
	let mut freertos = FreeRtos;
	unsafe {
		//vTaskDelete(core::ptr::null_mut());
		loop {
			//ets_run();
			
			freertos.delay_ms(4000u16);
		}
	}
	
	/*
	unsafe {
		xTaskCreatePinnedToCore(
			Some(raw_task1),
			CStr::from_bytes_with_nul_unchecked(b"task1\0").as_ptr(),
			2048,
			core::ptr::null_mut(),
			1, // Priority
			core::ptr::null_mut(),
			0, // CORE
		);
		xTaskCreatePinnedToCore(
			Some(raw_task2),
			CStr::from_bytes_with_nul_unchecked(b"task2\0").as_ptr(),
			2048,
			core::ptr::null_mut(),
			1, // Priority
			core::ptr::null_mut(),
			1, // CORE
		);
	}
	
	unsafe {
		vTaskDelete(core::ptr::null_mut());
		loop {
			ets_run();
		}
	}*/
}

/*
unsafe extern "C" fn raw_task1(_arg1: *mut c_void) {
	#[allow(unused_unsafe)]
	unsafe {
		esp_task_wdt_add(core::ptr::null_mut());
	}
	let _result = task1().unwrap();
}

unsafe extern "C" fn raw_task2(_arg1: *mut c_void) {
	#[allow(unused_unsafe)]
	unsafe {
		esp_task_wdt_add(core::ptr::null_mut());
	}
	let _result = task2().unwrap();
}

#[inline(always)]
fn task1() -> Result<(), EspError> {
	let mut gpio_led = {
		let mut per_lock = unsafe { PERIPHERALS.get_unchecked() }.lock();
		
		core::mem::replace(
			&mut per_lock.pins.gpio2, 
			unsafe { Gpio2::<Unknown>::new() }
		).into_output()?
	};
	
	
	let mut freertos = FreeRtos;
	let mut is_high = false;
	loop {
		//println!("state: {:?}, kernel: {:?}", is_high, core());
		if is_high {
			gpio_led.set_low()?;
			is_high = false;
		}else {
			gpio_led.set_high()?;
			is_high = true;
		}
		
		unsafe {
			esp_task_wdt_reset();
			freertos.delay_ms(1000u16);
		}
	}
}


#[inline(always)]
fn task2() -> Result<(), EspError> {
	let mut freertos = FreeRtos;
	
	/*let mut freertos = FreeRtos;
	loop {
		println!("kernel: {:?}", core());
		unsafe {
			esp_task_wdt_reset();
			freertos.delay_ms(1000u16);
		}
	}*/
	
	/*
	let mut connector = BleConnector {};
	let mut buff: [u8; 1024] = unsafe { std::mem::zeroed() };
	loop {
		if let Ok(size) = connector.read(&mut buff) {
			if size != 0 {
				let slice = &buff[..size];
				println!("{:?}", slice);
			}
		}
		
		unsafe {
			esp_task_wdt_reset();
			freertos.delay_ms(1000u16);
		}
	}*/
	
	/*let mut map = HashMap::with_capacity(1024);
	map.insert("test", String::from("test"));
	println!("{:?}", map);*/
	
	loop {
		unsafe {
			esp_task_wdt_reset();
			freertos.delay_ms(1000u16);
		}
	}
	
	//Ok(())
}
*/