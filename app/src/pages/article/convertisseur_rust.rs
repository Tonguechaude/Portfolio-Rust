use leptos::prelude::*;

#[component]
pub fn ConvertisseurRustArticlePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12 space-y-6 text-theme-primary">
            <h1 class="text-4xl font-bold">"Convertisseur Rust Dockerisé"</h1>

            <p class="text-lg">
                "Un convertisseur numérique en Rust permettant de comparer les performances avec une version équivalente en C. Le projet est entièrement dockerisé pour simplifier l'exécution."
                <a href="https://github.com/Tonguechaude/Convertisseur-Rust" class="text-blue-600 underline" target="_blank">"Voir le code source sur GitHub"</a>
            </p>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Présentation du Projet"</h2>
                <p>
                    "Ce convertisseur permet de convertir un nombre dans différentes bases : binaire, octal, décimal et hexadécimal. Il s’agit d’un projet d’apprentissage de Rust, basé sur un convertisseur en C réalisé auparavant. Il est également dockerisé pour être exécuté facilement sur n’importe quelle machine sans configuration préalable."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Fonctionnement Général"</h2>
                <p>"Le programme propose un menu interactif dans le terminal où l'utilisateur peut choisir le type de conversion. Ensuite, le nombre est converti et affiché dans la base choisie."</p>
                <pre class="bg-zinc-900 text-green-200 p-4 rounded-md overflow-x-auto text-sm">
                    <code>
                        {r###"
fn welcome_screen() {
    loop {
        screen_cleaner();
        cool_tag();
        let a = time::Duration::from_millis(4000);
        sleep(a);

        screen_cleaner();
        println!(\"-------------------------------------------\");
        println!(\">>> Welcome to Number System Converter <<<\");
        println!(\"------------------------------------------- \\n\");

        println!(\">> Select Conversion Type:\");
        println!(\"> 1. Binary Conversion\");
        println!(\"> 2. Decimal Conversion\");
        println!(\"> 3. Octal Conversion\");
        println!(\"> 4. Hexadecimal Conversion\");
        println!(\"> 5. Exit the Program \\n \\n\");

        println!(\"Enter the number & Hit ENTER: \");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(\"Failed to read line\");

        let choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(\"Error: the number must be between 1 to 5.\");
                println!(\"Press any key to continue...\");
                wait_for_keypress();
                continue;
            }
        };

        match choice {
            1 => user_input(1),
            2 => user_input(2),
            3 => user_input(3),
            4 => user_input(4),
            5 => exit_screen(),
            _ => {
                println!(\"\\nError: the number must be between 1 to 5.\\n\");
                println!(\"Press any key to continue...\");
                wait_for_keypress();
                welcome_screen();
            }
        }
    }
}
"###}
                    </code>
                </pre>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Tests Unitaires"</h2>
                <p>"Le projet contient également des tests unitaires pour garantir le bon fonctionnement des conversions entre les différentes bases."</p>
                <pre class="bg-zinc-900 text-green-200 p-4 rounded-md overflow-x-auto text-sm">
                    <code>
                        {r###"
use Convertisseur_Rust::convertisseur;

#[test]
fn test_binary_to_octal() {
    assert_eq!(convertisseur::binary_to_octal(101101), 55);
    assert_eq!(convertisseur::binary_to_octal(111111111111111), 77777);
    assert_eq!(convertisseur::binary_to_octal(110010), 62);
    assert_eq!(convertisseur::binary_to_octal(101010), 52);
}
"###}
                    </code>
                </pre>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Dockerisation du Projet"</h2>
                <p>"Un Dockerfile permet d’exécuter l’application sans avoir besoin de compiler le code localement. L’image est basée sur Ubuntu 24.04 et lance directement le binaire Rust."</p>
                <pre class="bg-zinc-900 text-blue-200 p-4 rounded-md overflow-x-auto text-sm">
                    <code>
                        {r###"
FROM ubuntu:24.04

RUN apt-get update && apt-get install --no-install-recommends -y bash \\
    && apt-get clean \\
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/convertisseur

COPY . /usr/src/convertisseur

CMD [\"./target/release/convertisseur_rust\"]
"###}
                    </code>
                </pre>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Script de Lancement Automatisé"</h2>
                <p>"Un script Bash `start.sh` permet de compiler le projet, exécuter les tests, construire l’image Docker et lancer le conteneur en mode interactif."</p>
                <pre class="bg-zinc-900 text-yellow-100 p-4 rounded-md overflow-x-auto text-sm">
                    <code>
                        {r###"
    cargo build --release \\
    && cargo test \\
    && docker build -t convertisseur_rust . \\
&& docker run --rm -it convertisseur_rust
"###}
                    </code>
                </pre>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Conclusion"</h2>
                <p>
                    "Ce projet a été une première prise en main de Rust, combinée à l’apprentissage de la conteneurisation avec Docker. Il m’a permis de mieux comprendre le fonctionnement des conversions de bases numériques, tout en découvrant les tests en Rust et la mise en production simplifiée via conteneur."
                </p>
            </section>
        </main>
    }
}
