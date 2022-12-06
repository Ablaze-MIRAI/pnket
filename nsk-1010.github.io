<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <title>Piano Nekomiya</title>
        <link rel="icon" type="image/svg+xml" href="./assets/logo-8f8f8f.svg">
        <link rel="apple-touch-icon" sizes="1024x1024" href="./assets/icon.png">
        <meta name="description" content="ぴねこのGitHub Pagesページ">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link href="//fonts.googleapis.com/css?family=Roboto:100,300,400,500,700|Roboto+Mono:400,700|Google+Sans+Text:400,500,700,900|Google+Sans:400,500,700,900|Google+Sans+Mono:400,500,700|Noto+Sans+JP:100,300,400,500,700" rel="stylesheet">
        <link href="//fonts.googleapis.com/css?family=Material+Icons+Sharp" rel="stylesheet">
        <link rel="stylesheet" href="https://unpkg.com/sanitize.css">
        <link rel="stylesheet" href="./css/style.css">
    </head>
    <body>
        <header>
            <h1><object type="image/svg+xml" data="assets/logo-ghpages.svg" width="22" height="22"></object>Piano Nekomiya</h1>
            <script>
                window.addEventListener('DOMContentLoaded', function(){
                    window.addEventListener('scroll', function(){
                        if ( window.scrollY == 0 ) {
                            document.querySelector('header').setAttribute('id', 'scroll0');
                        } else {
                            document.querySelector('header').setAttribute('id', 'scrollon');
                        }
                    });
                });
            </script>
        </header>
        <main>
            <h3>About</h3>
            <p>2006/8/10 島根県立中央病院(出雲市)生まれ、松江市北西部在住</p>
            <p>Ablaze所属</p>
            <h3>Links</h3>
            <div id="result" class="grid">
                <script type="text/javascript">
                    fetch('./pages.json')
                        .then(response => response.text())
                        .then(data => writekun(data));
                    function writekun(json) {
                        let pages = JSON.parse(json);
                        for (var i=0;i<pages.length;i++) {
                            var html = '<a style="text-decoration: none;" href="' +
                                pages[i].url +
                                '"><section><div class="card"><div class="img" style="background-image: url(' +
                                pages[i].image +
                                ')"></div><div class="card-text"><p>' +
                                pages[i].title +
                                '</p></div></section></a>';
                            var element = document.getElementById("result");
                            element.insertAdjacentHTML("beforeend", html);
                        }
                    }
                </script>
            </div>
        </main>
        <footer>
            <p style="text-align: right;"><span class="material-icons-sharp" style="vertical-align: -0.3rem;">copyright</span>2021&nbsp;Piano&nbsp;Nekomiya.</p>
        </footer>
    </body>
</html>