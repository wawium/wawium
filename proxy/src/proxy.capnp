@0xfd24954870bd3739;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("wawium");

struct Timezone {
    utcOffset @0 :UInt8;
}

struct Date {
    year @0 :Int16;
    month @1 :UInt8;
    day @2 :UInt8;
    timezone @3 :Timezone;
}

struct Invoice {
    creationDate @0 :Date;
}

interface Proxy {
    hello @0 (name :Text) -> (greetings :Text);

    listInvoices @1 () -> (invoiceList :List(Invoice));
}