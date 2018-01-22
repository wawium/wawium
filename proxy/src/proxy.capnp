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

interface Proxy {
    hello @0 (name :Text) -> (greetings :Text);

    struct Credentials {
        username @0 :Text;
        password @1 :Text;
    }

    interface Mission {
        listInvoices @0 () -> (list :List(Invoice));
    }

    interface Invoice {
        struct Invoice {
            creationDate @0 :Date;
        }

        listInvoices @0 () -> (list :List(Invoice));
    }
}