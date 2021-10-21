use cookie::Cookie;
use http::Uri;

mod querystring;

use querystring::querify;

fn main() {
    println!("hello world");
    let raw_str = r#"_sp_id.6552=2958745b74319f0b.1629716567.63.1631546600.1631524843.e3b52709-548f-4294-a25b-0e455d9f3b59; intercom-id-kkyuzh22=61f8e14a-d5a2-4473-b42e-d531251b2438; intercom-session-kkyuzh22=cDJMWGphWHpQd0ZrRFgyNnowVGxSeUZwRVorenJiZ2d2eWpqUUVXQ3JjeTdiOFUwc0pwZDJtazhOSG1raGVYai0tOUpjclVjbXcwWmNKWjlnRXdydHFSQT09--da21560301b01f51cae4f348fe7b1780d622c5a5; _gcl_au=1.1.2139484071.1629727332; IR_PI=b6bddef1-041a-11ec-aa8d-4dc937371914%7C1631633000217; _biz_uid=50968bc39c74437ad089c7cfee05ed23; _biz_nA=218; _biz_pendingA=%5B%5D; _rdt_uuid=1629727332237.74252bfb-57da-42d2-9284-da7131d766e1; _scid=89db7974-bda4-4435-b6d4-bd06d8d9c789; _ga=GA1.2.1771823348.1629727334; _mkto_trk=id:307-OAT-968&token:_mch-datacamp.com-1629727334999-97145; _biz_flagsA=%7B%22Version%22%3A1%2C%22Mkto%22%3A%221%22%2C%22XDomain%22%3A%221%22%2C%22ViewThrough%22%3A%221%22%2C%22Frm%22%3A%221%22%7D; _cioid=3744979; cb_user_id=null; cb_group_id=null; cb_anonymous_id=%224aa50eb2-f79d-42ce-922a-37be1e4bc919%22; _hjid=55a118e2-f95e-404e-a529-9def1ef99b94; dc_consent={"version":"1.0.0","essential":1,"functional":1,"performance":1,"advertisement":1}; _fbp=fb.1.1629815773291.303639409; _sctr=1|1630965600000; __zlcmid=15kkHVNvRsrd8dY; cf_clearance=xTinxTKloUaJr2jS1NBpvHD6BZJQ9RgiBW9EsMq0RnU-1631524693-0-150; _gid=GA1.2.892506696.1631520461; IR_gbd=datacamp.com; IR_13294=1631546600217%7C0%7C1631546600217%7C%7C; _dct=eyJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJodHRwczovL3d3dy5kYXRhY2FtcC5jb20iLCJqdGkiOiIzNzQ0OTc5LTYyYTViN2MxNjE2ZmM4OGFmMWFlNzQ5NTgwNzUyZGQ4YzYwOTdmNzg1ZWQ5MjVhZjc3NDVjNjFlNzY4OCIsInVzZXJfaWQiOjM3NDQ5NzksImV4cCI6MTYzOTM4NzExMX0.DRsjodE9OMtPCokUIjdUnOH-UNIk9d0-FQmZo2q-j6RG3f7L6fsnCL51aqG8WWxLCfTy3wd_jNi0otJa5D5QANuJrinLvOkoAv1OQEZdppAvd3ype3OSTAkvmOOZaFIBl645DCSlK_TCRtvkSso7shAXOxnbRXccp5K6b9gjIEPoESr9Q3mpWPASMNAweHmbrAFKRqNcdCIl2k6SyOZ7rcfcV2p8BCqAvw8QnmuDoUBsxlNSZ25D01rgMRp98AtQSlgqRrAuiVMxydrO12Ry-3Cje-HqJf4AYyW3sUQC0JUYWDe4rPJx_0BToMtQk04ELJ9A66YFPlWYTHqaV0nI2Q; __cf_bm=IkPg9W0VL.QVCz.26dXONofAbX2YYZWSz9.6NjAwqSg-1631546599-0-AdPs8aWY+agugAOcvy7a7O7y9VKsaR0g/fhA4Ec+gZYCbnYfrUi6sxzetMEKGS83hXkwX0DSOdkTzwkkiLCfBsI=; _sp_ses.6552=*; _biz_sid=154b2e; _uetsid=abf40720146911ecb5593304a83b0269; _uetvid=b6afd350041a11ec88800964eccd549c; _dc_gtm_UA-39297847-1=1; _hjIncludedInSessionSample=0; _hjAbsoluteSessionInProgress=0"#;
    let dct = raw_str
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Cookie::parse_encoded)
        .filter_map(|c| c.ok())
        .find(|c| c.name() == "_dct");

    if dct.is_some() {
        println!("DCT COOKIE VALUE {}", dct.unwrap().value());
    }

    let parsed = raw_str
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Cookie::parse_encoded)
        .filter_map(|c| c.ok());

    for cookie in parsed {
        if cookie.name() == "_dct" {
            println!("Found dct Cookie {}={}", cookie.name(), cookie.value());
        } else {
            println!("Not the dct cookie {}={}", cookie.name(), cookie.value());
        }
    }

    let uri = "/hello?sid=123".parse::<Uri>().unwrap_or_default();

    let sid = uri
        .query()
        .map(|query| querify(query))
        .unwrap_or_default()
        .into_iter()
        .filter(|part| part.0 == "sid")
        .nth(0);

    match sid {
        Some(s) => {
            println!("Found a query {}:{}", s.0, s.1);
        }
        None => {
            println!("No query in this shit");
        }
    }
}
