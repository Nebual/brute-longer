extern crate iron;

use self::iron::prelude::*;
use self::iron::status;
use brutelonger;

pub fn launch_webserver(binding: &str, url_base: &str, max: u64) {

    let my_url_base = url_base.to_owned();

    Iron::new(move |req: &mut Request| -> IronResult<Response> {
        let chunks = req.url.path();
        if chunks.len() == 3 {
            let target = brutelonger::words_to_int(chunks[1].to_owned(), chunks[2].to_owned(), max);
            let url = iron::Url::parse(format!("{}/{}-{}", my_url_base, chunks[0], target).as_str()).unwrap();
            Ok(Response::with((status::PermanentRedirect, iron::modifiers::Redirect(url))))
        } else if chunks.len() == 2 {
            let target: u64 = chunks[1].parse().unwrap_or(0);
            if target == 0 {
                Ok(Response::with((status::BadRequest, ":(")))
            } else {
                let lines = brutelonger::brute_words_from_int(target, max, 10);
                let joined = lines.iter()
                    .map(|&ref line| {
                        let mut words = line.split_whitespace();
                        let adj = words.next().unwrap();
                        let noun = words.next().unwrap();
                        format!("<a class='btn' href='/{}/{}/{}'>{} {}</a><br>", chunks[0], adj, noun, adj, noun)
                    })
                    .fold("".to_string(), |accumulator, new_str| {
                        accumulator + &new_str + "\n"
                    });
                let mut resp = Response::with((status::Ok, format!("\
                    <link href='https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css' rel='stylesheet' crossorigin='anonymous'>\
                    <div class='container'>\
                        <h1>The following word pairs all correspond with {}-{}:</h1>\
                        {}\
                    </div>\
                    ", chunks[0], target, joined)));
                resp.headers.set(iron::headers::ContentType::html());
                Ok(resp)
            }
        } else {
            Ok(Response::with((status::BadRequest, ":(")))
        }

    }).http(binding).unwrap();
}

/* Sample Nginx /etc/nginx/sites-enabled/brute-longer.conf location block:
    location / {
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Host $http_host;
        proxy_pass http://localhost:3000;
        proxy_redirect off;
    }
*/
