#![allow(non_camel_case_types)]

extern crate libc;
use libc::{c_char, c_int, c_void, c_long, c_longlong, FILE, mode_t, size_t};

pub const RD_KAFKA_PARTITION_UA: c_int = -1;
pub const RD_KAFKA_OFFSET_BEGINNING: c_int = -2;
pub const RD_KAFKA_OFFSET_END: c_int = -1;
pub const RD_KAFKA_OFFSET_STORED: c_int = -1000;
pub const RD_KAFKA_OFFSET_TAIL_BASE: c_int = -2000;

#[repr(C)]
#[derive(Copy)]
pub enum rd_kafka_type_t {
    RD_KAFKA_PRODUCER,
    RD_KAFKA_CONSUMER
}

#[repr(C)]
#[derive(Copy)]
pub enum rd_kafka_resp_err_t {
    /* Internal errors to rdkafka: */
    RD_KAFKA_RESP_ERR__BEGIN = -200,     /* begin internal error codes */
    RD_KAFKA_RESP_ERR__BAD_MSG = -199,   /* Received message is incorrect */
    RD_KAFKA_RESP_ERR__BAD_COMPRESSION = -198, /* Bad/unknown compression */
    RD_KAFKA_RESP_ERR__DESTROY = -197,   /* Broker is going away */
    RD_KAFKA_RESP_ERR__FAIL = -196,      /* Generic failure */
    RD_KAFKA_RESP_ERR__TRANSPORT = -195, /* Broker transport error */
    RD_KAFKA_RESP_ERR__CRIT_SYS_RESOURCE = -194, /* Critical system resource
     * failure */
    RD_KAFKA_RESP_ERR__RESOLVE = -193,   /* Failed to resolve broker */
    RD_KAFKA_RESP_ERR__MSG_TIMED_OUT = -192, /* Produced message timed out*/
    RD_KAFKA_RESP_ERR__PARTITION_EOF = -191, /* Reached the end of the
     * topic+partition queue on
     * the broker.
     * Not really an error. */
    RD_KAFKA_RESP_ERR__UNKNOWN_PARTITION = -190, /* Permanent:
     * Partition does not
     * exist in cluster. */
    RD_KAFKA_RESP_ERR__FS = -189,        /* File or filesystem error */
    RD_KAFKA_RESP_ERR__UNKNOWN_TOPIC = -188, /* Permanent:
     * Topic does not exist
     * in cluster. */
    RD_KAFKA_RESP_ERR__ALL_BROKERS_DOWN = -187, /* All broker connections
     * are down. */
    RD_KAFKA_RESP_ERR__INVALID_ARG = -186,  /* Invalid argument, or
     * invalid configuration */
    RD_KAFKA_RESP_ERR__TIMED_OUT = -185,    /* Operation timed out */
    RD_KAFKA_RESP_ERR__QUEUE_FULL = -184,   /* Queue is full */
    RD_KAFKA_RESP_ERR__ISR_INSUFF = -183,   /* ISR count < required.acks */
    RD_KAFKA_RESP_ERR__END = -100,       /* end internal error codes */

    /* Standard Kafka errors: */
    RD_KAFKA_RESP_ERR_UNKNOWN = -1,
    RD_KAFKA_RESP_ERR_NO_ERROR = 0,
    RD_KAFKA_RESP_ERR_OFFSET_OUT_OF_RANGE = 1,
    RD_KAFKA_RESP_ERR_INVALID_MSG = 2,
    RD_KAFKA_RESP_ERR_UNKNOWN_TOPIC_OR_PART = 3,
    RD_KAFKA_RESP_ERR_INVALID_MSG_SIZE = 4,
    RD_KAFKA_RESP_ERR_LEADER_NOT_AVAILABLE = 5,
    RD_KAFKA_RESP_ERR_NOT_LEADER_FOR_PARTITION = 6,
    RD_KAFKA_RESP_ERR_REQUEST_TIMED_OUT = 7,
    RD_KAFKA_RESP_ERR_BROKER_NOT_AVAILABLE = 8,
    RD_KAFKA_RESP_ERR_REPLICA_NOT_AVAILABLE = 9,
    RD_KAFKA_RESP_ERR_MSG_SIZE_TOO_LARGE = 10,
    RD_KAFKA_RESP_ERR_STALE_CTRL_EPOCH = 11,
    RD_KAFKA_RESP_ERR_OFFSET_METADATA_TOO_LARGE = 12
}

#[repr(C)]
#[derive(Copy)]
pub enum rd_kafka_conf_res_t {
    RD_KAFKA_CONF_UNKNOWN = -2, /* Unknown configuration name. */
    RD_KAFKA_CONF_INVALID = -1, /* Invalid configuration value. */
    RD_KAFKA_CONF_OK = 0        /* Configuration okay */
}

pub enum rd_kafka_s { }
pub type rd_kafka_t = rd_kafka_s;
pub enum rd_kafka_topic_s { }
pub type rd_kafka_topic_t = rd_kafka_topic_s;
pub enum rd_kafka_conf_s { }
pub type rd_kafka_conf_t = rd_kafka_conf_s;
pub enum rd_kafka_topic_conf_s { }
pub type rd_kafka_topic_conf_t = rd_kafka_topic_conf_s;
pub enum rd_kafka_queue_s { }
pub type rd_kafka_queue_t = rd_kafka_queue_s;

#[repr(C)]
#[derive(Copy)]
pub struct rd_kafka_message_s {
    pub err: rd_kafka_resp_err_t,
    pub rkt: *mut rd_kafka_topic_t,
    pub partition: c_int,
    pub payload: *mut c_void,
    pub len: size_t,
    pub key: *mut c_void,
    pub key_len: size_t,
    pub offset: c_longlong,
    pub _private: *mut c_void
}
impl ::std::default::Default for rd_kafka_message_s {
    fn default() -> rd_kafka_message_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type rd_kafka_message_t = rd_kafka_message_s;

#[repr(C)]
#[derive(Copy)]
pub struct rd_kafka_metadata_broker {
    pub id: c_int,
    pub host: *mut ::libc::c_char,
    pub port: ::libc::c_int,
}
impl ::std::default::Default for rd_kafka_metadata_broker {
    fn default() -> rd_kafka_metadata_broker {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type rd_kafka_metadata_broker_t = rd_kafka_metadata_broker;

#[repr(C)]
#[derive(Copy)]
pub struct rd_kafka_metadata_partition {
    pub id: c_int,
    pub err: rd_kafka_resp_err_t,
    pub leader: c_int,
    pub replica_cnt: ::libc::c_int,
    pub replicas: *mut c_int,
    pub isr_cnt: ::libc::c_int,
    pub isrs: *mut c_int,
}
impl ::std::default::Default for rd_kafka_metadata_partition {
    fn default() -> rd_kafka_metadata_partition {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type rd_kafka_metadata_partition_t = rd_kafka_metadata_partition;

#[repr(C)]
#[derive(Copy)]
pub struct rd_kafka_metadata_topic {
    pub topic: *mut c_char,
    pub partition_cnt: c_int,
    pub partitions: *mut rd_kafka_metadata_partition,
    pub err: rd_kafka_resp_err_t,
}
impl ::std::default::Default for rd_kafka_metadata_topic {
    fn default() -> rd_kafka_metadata_topic {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type rd_kafka_metadata_topic_t = rd_kafka_metadata_topic;

#[repr(C)]
#[derive(Copy)]
pub struct rd_kafka_metadata {
    pub broker_cnt: c_int,
    pub brokers: *mut rd_kafka_metadata_broker,
    pub topic_cnt: c_int,
    pub topics: *mut rd_kafka_metadata_topic,
    pub orig_broker_id: c_int,
    pub orig_broker_name: *mut c_char,
}
impl ::std::default::Default for rd_kafka_metadata {
    fn default() -> rd_kafka_metadata {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type rd_kafka_metadata_t = rd_kafka_metadata;


#[link(name = "rdkafka")]
#[link(name = "z")]
#[link(name = "pthread")]
//not available on osx
//#[link(name = "rt")]
extern {

    pub fn rd_kafka_version() -> c_int;

    pub fn rd_kafka_version_str() -> *const c_char;

    pub fn rd_kafka_err2str(err: rd_kafka_resp_err_t) -> *const c_char;

    pub fn rd_kafka_errno2err(errnox: c_int) -> rd_kafka_resp_err_t;

    pub fn rd_kafka_message_destroy(rkmessage: *mut rd_kafka_message_t);

    pub fn rd_kafka_conf_new() -> *mut rd_kafka_conf_t;

    pub fn rd_kafka_conf_destroy(conf: *mut rd_kafka_conf_t);

    pub fn rd_kafka_conf_dup(conf: *const rd_kafka_conf_t) -> *mut rd_kafka_conf_t;

    pub fn rd_kafka_conf_set(conf: *mut rd_kafka_conf_t,
                             name: *const c_char,
                             value: *const c_char,
                             errstr: *mut c_char, errstr_size: size_t)
                             -> rd_kafka_conf_res_t;

    pub fn rd_kafka_conf_set_dr_cb(conf: *mut rd_kafka_conf_t,
                                   dr_cb: Option<extern "C" fn
                                   (rk: *mut rd_kafka_t,
                                    payload: *mut c_void,
                                    len: size_t,
                                    err: rd_kafka_resp_err_t,
                                    opaque: *mut c_void,
                                    msg_opaque: *mut c_void)>);
    
    pub fn rd_kafka_conf_set_dr_msg_cb(conf: *mut rd_kafka_conf_t,
                                       dr_msg_cb: Option<extern "C" fn
                                       (rk: *mut rd_kafka_t,
                                        rkmessage: *const rd_kafka_message_t,
                                        opaque: *mut c_void)>);

    pub fn rd_kafka_conf_set_error_cb(conf: *mut rd_kafka_conf_t,
                                      error_cb: Option<extern "C" fn
                                      (rk: *mut rd_kafka_t,
                                       err: c_int,
                                       reason: *const c_char,
                                       opaque: *mut c_void)>);
    
    pub fn rd_kafka_conf_set_log_cb(conf: *mut rd_kafka_conf_t,
                                    log_cb: Option<extern "C" fn
                                    (rk: *const rd_kafka_t,
                                     level: c_int,
                                     fac: *const c_char,
                                     buf: *const c_char)>);

    pub fn rd_kafka_conf_set_stats_cb(conf: *mut rd_kafka_conf_t,
                                      stats_cb: Option<extern "C" fn
                                      (rk: *mut rd_kafka_t,
                                       json: *mut c_char,
                                       json_len: size_t,
                                       opaque:*mut c_void)
                                       -> c_int>);
    
    pub fn rd_kafka_conf_set_socket_cb(conf: *mut rd_kafka_conf_t,
                                       socket_cb: Option<extern "C" fn
                                       (domain: c_int,
                                        _type: c_int,
                                        protocol: c_int,
                                        opaque: *mut c_void)
                                        -> c_int>);
    
    pub fn rd_kafka_conf_set_open_cb(conf: *mut rd_kafka_conf_t,
                                     open_cb: Option<extern "C" fn
                                     (pathname: *const c_char,
                                      flags: c_int,
                                      mode: mode_t,
                                      opaque: *mut c_void)
                                      -> c_int>);

    pub fn rd_kafka_conf_set_opaque(conf: *mut rd_kafka_conf_t,
                                    opaque: *mut c_void);

    pub fn rd_kafka_opaque(rk: *const rd_kafka_t) -> *mut c_void;

    pub fn rd_kafka_conf_dump(conf: *mut rd_kafka_conf_t, cntp: *mut size_t)
                              -> *mut *const c_char;

    pub fn rd_kafka_topic_conf_dump(conf: *mut rd_kafka_topic_conf_t,
                                    cntp: *mut size_t)
                                    -> *mut *const c_char;

    pub fn rd_kafka_conf_dump_free(arr: *mut *const c_char,
                                   cnt: size_t);

    pub fn rd_kafka_conf_properties_show(fp: *mut FILE);

    pub fn rd_kafka_topic_conf_new() -> *mut rd_kafka_topic_conf_t;

    pub fn rd_kafka_topic_conf_dup(conf: *const rd_kafka_topic_conf_t)
                                   -> *mut rd_kafka_topic_conf_t;

    pub fn rd_kafka_topic_conf_destroy(topic_conf:
                                       *mut rd_kafka_topic_conf_t);

    pub fn rd_kafka_topic_conf_set(conf: *mut rd_kafka_topic_conf_t,
                                   name: *const c_char,
                                   value: *const c_char,
                                   errstr: *mut c_char,
                                   errstr_size: size_t)
                                   -> rd_kafka_conf_res_t;

    pub fn rd_kafka_topic_conf_set_opaque(conf: *mut rd_kafka_topic_conf_t,
                                          opaque: *mut c_void);

    pub fn rd_kafka_topic_conf_set_partitioner_cb(topic_conf:
                                                  *mut rd_kafka_topic_conf_t,
                                                  partitioner:
                                                  Option<extern "C" fn
                                                  (rkt:
                                                   *const rd_kafka_topic_t,
                                                   keydata:
                                                   *const c_void,
                                                   keylen:
                                                   size_t,
                                                   partition_cnt:
                                                   c_int,
                                                   rkt_opaque:
                                                   *mut c_void,
                                                   msg_opaque:
                                                   *mut c_void)
                                                   ->
                                                  c_int>);

    pub fn rd_kafka_topic_partition_available(rkt: *const rd_kafka_topic_t,
                                              partition: c_int)
                                              -> c_int;

    pub fn rd_kafka_msg_partitioner_random(rkt: *const rd_kafka_topic_t,
                                           key: *const c_void,
                                           keylen: size_t,
                                           partition_cnt: c_int,
                                           opaque: *mut c_void,
                                           msg_opaque: *mut c_void)
                                           -> c_int;

    pub fn rd_kafka_new(_type: rd_kafka_type_t, conf: *mut rd_kafka_conf_t,
                        errstr: *mut c_char, errstr_size: size_t)
                        -> *mut rd_kafka_t;

    pub fn rd_kafka_destroy(rk: *mut rd_kafka_t);

    pub fn rd_kafka_name(rk: *const rd_kafka_t) -> *const c_char;

    pub fn rd_kafka_topic_new(rk: *mut rd_kafka_t,
                              topic: *const c_char,
                              conf: *mut rd_kafka_topic_conf_t)
                              -> *mut rd_kafka_topic_t;

    pub fn rd_kafka_topic_destroy(rkt: *mut rd_kafka_topic_t);

    pub fn rd_kafka_topic_name(rkt: *const rd_kafka_topic_t)
                               -> *const c_char;

    pub fn rd_kafka_queue_new(rk: *mut rd_kafka_t) -> *mut rd_kafka_queue_t;

    pub fn rd_kafka_queue_destroy(rkqu: *mut rd_kafka_queue_t);

    pub fn rd_kafka_consume_start(rkt: *mut rd_kafka_topic_t,
                                  partition: c_int, offset: c_longlong)
                                  -> c_int;

    pub fn rd_kafka_consume_start_queue(rkt: *mut rd_kafka_topic_t,
                                        partition: c_int, offset: c_longlong,
                                        rkqu: *mut rd_kafka_queue_t)
                                        -> c_int;

    pub fn rd_kafka_consume_stop(rkt: *mut rd_kafka_topic_t,
                                 partition: c_int) -> c_int;

    pub fn rd_kafka_consume(rkt: *mut rd_kafka_topic_t, partition: c_int,
                            timeout_ms: c_int)
                            -> *mut rd_kafka_message_t;

    pub fn rd_kafka_consume_batch(rkt: *mut rd_kafka_topic_t,
                                  partition: c_int,
                                  timeout_ms: c_int,
                                  rkmessages: *mut *mut rd_kafka_message_t,
                                  rkmessages_size: size_t) -> c_long;

    pub fn rd_kafka_consume_callback(rkt: *mut rd_kafka_topic_t,
                                     partition: c_int,
                                     timeout_ms: c_int,
                                     consume_cb:
                                     Option<extern "C" fn
                                     (rkmessage:
                                      *mut rd_kafka_message_t,
                                      opaque:
                                      *mut c_void)>,
                                     opaque: *mut c_void)
                                     -> c_int;

    pub fn rd_kafka_consume_queue(rkqu: *mut rd_kafka_queue_t,
                                  timeout_ms: c_int)
                                  -> *mut rd_kafka_message_t;

    pub fn rd_kafka_consume_batch_queue(rkqu: *mut rd_kafka_queue_t,
                                        timeout_ms: c_int,
                                        rkmessages:
                                        *mut *mut rd_kafka_message_t,
                                        rkmessages_size: size_t) -> c_long;

    pub fn rd_kafka_consume_callback_queue(rkqu: *mut rd_kafka_queue_t,
                                           timeout_ms: c_int,
                                           consume_cb:
                                           Option<extern "C" fn
                                           (rkmessage:
                                            *mut rd_kafka_message_t,
                                            opaque:
                                            *mut c_void)>,
                                           opaque: *mut c_void)
                                           -> c_int;

    pub fn rd_kafka_offset_store(rkt: *mut rd_kafka_topic_t,
                                 partition: c_int, offset: c_longlong)
                                 -> rd_kafka_resp_err_t;

    pub fn rd_kafka_produce(rkt: *mut rd_kafka_topic_t, partitition: c_int,
                            msgflags: c_int,
                            payload: *mut c_void, len: size_t,
                            key: *const c_void, keylen: size_t,
                            msg_opaque: *mut c_void) -> c_int;

    pub fn rd_kafka_produce_batch(rkt: *mut rd_kafka_topic_t,
                                  partition: c_int, msgflags: c_int,
                                  rkmessages: *mut rd_kafka_message_t,
                                  message_cnt: c_int)
                                  -> c_int;

    pub fn rd_kafka_metadata(rk: *mut rd_kafka_t, all_topics: c_int,
                             only_rkt: *mut rd_kafka_topic_t,
                             metadatap: *mut *const rd_kafka_metadata,
                             timeout_ms: c_int)
                             -> rd_kafka_resp_err_t;

    pub fn rd_kafka_metadata_destroy(metadata:
                                     *const rd_kafka_metadata);

    pub fn rd_kafka_poll(rk: *mut rd_kafka_t, timeout_ms: c_int)
                         -> c_int;
    
    pub fn rd_kafka_brokers_add(rk: *mut rd_kafka_t,
                                brokerlist: *const c_char)
                                -> c_int;

    pub fn rd_kafka_set_logger(rk: *mut rd_kafka_t,
                               func: Option<extern "C" fn
                               (rk: *const rd_kafka_t,
                                level: c_int,
                                fac: *const c_char,
                                buf: *const c_char)>);

    pub fn rd_kafka_set_log_level(rk: *mut rd_kafka_t, level: c_int);

    pub fn rd_kafka_log_print(rk: *const rd_kafka_t, level: c_int,
                              fac: *const c_char,
                              buf: *const c_char);

    pub fn rd_kafka_log_syslog(rk: *const rd_kafka_t, level: c_int,
                               fac: *const c_char,
                               buf: *const c_char);

    pub fn rd_kafka_outq_len(rk: *mut rd_kafka_t) -> c_int;

    pub fn rd_kafka_dump(fp: *mut FILE, rk: *mut rd_kafka_t);

    pub fn rd_kafka_thread_cnt() -> c_int;

    pub fn rd_kafka_wait_destroyed(timeout_ms: c_int)
                                   -> c_int;
    
}

#[test]
fn it_works() {
    use std::ffi::{c_str_to_bytes};

    unsafe {
        assert!(rd_kafka_version() == 0x00080500);
        let version_str = rd_kafka_version_str();
        let version_str_b = c_str_to_bytes(&version_str);
        let version_str_s = String::from_utf8_unchecked(version_str_b.to_vec());
        println!("version_str_s|{}|", version_str_s);
        assert!(version_str_s == "0.8.5");
    }
}
