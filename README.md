Un utilitaire pour la génération de rapport laboratoire rapide en LATEX. 

## Utilisation
```
latex-gen <fichier d'input>
```

## Format du fichier d'input
```
__SECTION__ Section avec un tableau
| V_n | I_n | R_n   |
|     |     | V_n/I_n |*3
|     |     | =\frac{V_n}{I_n} |

__SECTION__ Section sans tableau
test
```
Chaque section doit commencer par `__SECTION__` suivis du titre de la section. Chaque section sera son propre `\item` dans le document latex. Un tableau a ensuite 3 parties : l'en-tête, le corps et l'exemple de calcul. Chaque colonne est séparé par le caractère `|`. Dans la première rangé (l'en-tête), mettre le nom de la colonne. Dans la seconde rangé (le corps) laisser vide si la cellule prend une valeur et mettre la formule (avec le nom des colonnes) si cette cellule a une valeur calculé. À la fin de cette rangé, indiquer `*n` où `n` est le nombre de rangé voulu dans le tableau. Finalement la dernière ligne est le même calcul qu'a la rangé précédente mais au format latex et sera utilisé dans l'exemple de calcul.

L'exemple précédent nous sort la sortie suivante:
```latex
====== HEADER ======
\usepackage[shortlabels]{enumitem}
\usepackage[xfp]{spreadtab}
\usepackage{tabu}
====== BODY ======
\begin{enumerate}[a) ]
\item \textbf{Section avec un tableau}
    \begin{figure}[hbt!]
        \centering
        \captionof{table}{Section avec un tableau}
        \begin{spreadtab}{{tabu}{|c|c|c|}}\hline
            @$ V_n $ & @$ I_n $ & @$ R_n $ \\\hline
            @$     $ & @$   $ & @$   $     \\\hline\hline
                     &        & a3/b3      \\\hline
                     &        & a4/b4      \\\hline
                     &        & a5/b5      \\\hline
        \end{spreadtab}
    \end{figure}

    Exemple de calcul pour $R_n$:
    \[ R_n = \frac{V_n}{I_n} = \]

\item \textbf{Section sans tableau}

\end{enumerate}
```