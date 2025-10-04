use leptos::prelude::*;

#[component]
pub fn GolJavaArticlePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12 space-y-6 text-theme-primary">
            <h1 class="text-4xl font-bold">"Jeu de la Vie en Java avec Swing"</h1>

            <p class="text-lg">
                "Implémentation du célèbre automate cellulaire de John Conway en Java, avec une interface graphique interactive basée sur Swing."
                <a href="https://github.com/Tonguechaude/GOL" class="text-blue-600 underline" target="_blank">"Voir le dépôt GitHub"</a>
            </p>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Contexte du projet"</h2>
                <p>
                    "Réalisé sur mon temps libre par intérêt pour le concept du Jeu de la Vie, ce projet a été l'occasion de construire un simulateur d'automate cellulaire avec une interface graphique intuitive et fonctionnelle en Java."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Structure du projet"</h2>
                <pre class="bg-zinc-900 text-green-200 p-4 rounded-md text-sm overflow-x-auto">
                    <code>{r###"
├── controlleur
│   └── ControlleurJeu.java
├── Main.java
├── model
│   ├── Cellule.java
│   ├── Grille.java
│   ├── JeuDeLaVie.java
│   └── Motif.java
├── README.md
└── vue
    ├── Fenetre.java
    └── Rendu.java
"###}</code>
                </pre>
                <ul class="list-disc ml-6">
                    <li><b>ControlleurJeu.java</b> : Logique du jeu et événements utilisateur.</li>
                    <li><b>Main.java</b>" : Point d’entrée de l’application."</li>
                    <li><b>model/</b>" : Cellule, grille, moteur de jeu, et chargement de motifs."</li>
                    <li><b>vue/</b>" : Fenêtre principale et rendu graphique des cellules."</li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Fonctionnalités clés"</h2>
                <ul class="list-disc ml-6">
                    <li>"Interface graphique interactive avec Swing."</li>
                    <li>"Contrôle de l'évolution (play/pause/reset, vitesse)."</li>
                    <li>"Chargement et affichage de motifs pré-définis."</li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Compétences développées"</h2>
                <ul class="list-disc ml-6">
                    <li>"Programmation orientée objet (POO) en Java."</li>
                    <li>"Application des principes SOLID."</li>
                    <li>"Conception d’interfaces avec Swing."</li>
                    <li>"Gestion des événements utilisateur."</li>
                    <li>"Implémentation des règles d’automates cellulaires."</li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Exemple de code : mise à jour de la grille"</h2>
                <pre class="bg-zinc-900 text-yellow-100 p-4 rounded-md text-sm overflow-x-auto">
                    <code>{r###"
public void maj() {
    Cellule[][] nouvelleGrille = new Cellule[lignes][colonnes];

    for (int i = 0; i < lignes; i++) {
        for (int j = 0; j < colonnes; j++) {
            int voisinesEnVies = compterVoisineEnVie(i, j);
            boolean enVie = cellules[i][j].estVivante()
                ? (voisinesEnVies == 2 || voisinesEnVies == 3)
                : (voisinesEnVies == 3);

            nouvelleGrille[i][j] = new Cellule(enVie);
        }
    }
    cellules = nouvelleGrille;
}
"###}</code>
                </pre>
                <p>Cette méthode applique les règles du Jeu de la Vie pour générer la prochaine génération de cellules.</p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Capture d’écran de l’interface"</h2>
                <img src="/img/gol.png" alt="Interface graphique du Jeu de la Vie" class="rounded-lg border shadow-lg" />
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Conclusion"</h2>
                <p>
                    "Ce projet m’a permis d’allier mes compétences en Java et en conception graphique tout en découvrant les subtilités d’un automate cellulaire. Une belle façon d'explorer à la fois la rigueur algorithmique et l'interactivité utilisateur."
                </p>
            </section>
        </main>
    }
}
