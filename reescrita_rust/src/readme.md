Exercício 1 — Verificar Primeiro

Complexidade: O(1)

Lógica do algoritmo:
O algoritmo recebe uma lista de elementos e verifica se ela está vazia. Se estiver vazia, retorna uma ausência de valor (None / None); caso contrário, acessa e retorna diretamente o primeiro elemento da coleção (índice 0).

Justificativa da complexidade:
Não importa se a lista possui 10 ou 10 milhões de elementos, a verificação do tamanho e o acesso a um índice específico de um array na memória ocorrem de forma direta. Como não existem laços de repetição (loops) ou recursão dependentes de n, o tempo de execução é constante.



Exercício 2 — Somar Lista

Complexidade: O(n)

Lógica do algoritmo:
Passa sequencialmente por todos os números contidos na lista, acumulando o valor de cada elemento dentro de uma variável mutável de controle (total), retornando a soma aritmética final.

Justificativa da complexidade:
Existe exatamente um loop simples (for) que itera sobre a coleção. O loop é executado exatamente n vezes, onde n é o número de elementos dentro da lista. Portanto, o tempo de execução cresce de forma linear e proporcional ao tamanho da entrada.



Exercício 3 — Busca Binária

Complexidade: O(logn)

Lógica do algoritmo:
Divide iterativamente o intervalo de busca na metade. Em cada iteração do laço, compara o elemento central com o valor alvo. Se o alvo for menor, descarta a metade superior; se for maior, descarta a metade inferior, até encontrar o índice ou esgotar o espaço amostral.

Justificativa da complexidade:
A cada ciclo do loop while, o espaço de busca é reduzido pela metade (n→2n​→4n​…). A operação matemática inversa de dividir sucessivamente por 2 é o logaritmo na base 2 (log2​n). Assim, o pior caso exige no máximo ≈log2​n iterações.



Exercício 4 — Pares com Soma

Complexidade: O(n2)

Lógica do algoritmo:
O algoritmo percorre a lista combinando cada elemento do índice i com todos os elementos subsequentes do índice j (j>i). Se a soma dos dois elementos for igual ao valor de alvo, o par é registrado.

Justificativa da complexidade:
Existem dois loops for aninhados. O primeiro loop roda n vezes. O segundo loop interno roda um número decrescente de vezes (n−1,n−2,…,1). O número total de iterações é dado pela soma da progressão aritmética:
2n(n−1)​=2n2−n​

Desprezando os termos de menor ordem e as constantes na análise Big-O, obtemos a classe quadrática.



Exercício 5 — Imprimir Pares e Pares

Complexidade: O(n2)

Lógica do algoritmo:
O algoritmo é composto por dois blocos independentes e sequenciais. O Bloco 1 realiza uma varredura linear imprimindo os elementos da lista individualmente. O Bloco 2 realiza uma varredura cruzada gerando todas as combinações de pares possíveis do conjunto (incluindo o elemento com ele mesmo).

Justificativa da complexidade:
Pela regra da soma da análise assintótica, calculamos a complexidade de blocos independentes somando-os: O(n) [Bloco 1]+O(n×n) [Bloco 2]=O(n+n2). Como o termo dominante determina a complexidade no pior caso, o algoritmo é simplificado para O(n2).



Exercício 6 — Potências de Dois

Complexidade: O(logn)

Lógica do algoritmo:
A função inicializa uma variável em 1 e, por meio de um loop while, imprime o seu valor corrente, multiplicando-o por 2 a cada rodada. O loop é interrompido assim que o valor atinge ou ultrapassa o limite estabelecido por n.

Justificativa da complexidade:
A variável de controle i cresce exponencialmente (1,2,4,8,16,…). O número de passos necessários para que 2k≥n é determinado aplicando o logaritmo em ambos os lados da equação, o que resulta em k=log2​n passos.



Exercício 7 — Fibonacci Recursivo

Complexidade: O(2n)

Lógica do algoritmo:
Calcula o n-ésimo termo da sequência de Fibonacci utilizando recursão de árvore. Se n≤1, retorna o próprio número (caso base); caso contrário, realiza duas novas chamadas recursivas para calcular os termos anteriores (n−1 e n−2) e soma-os.

Justificativa da complexidade:
Cada chamada da função que não seja um caso base gera outras duas chamadas subsequentes. Isso cria uma árvore de recursão binária com profundidade máxima proporcional a n. O número total de nós nesta árvore cresce exponencialmente, resultando em uma complexidade de tempo de pior caso de O(2n).



Exercício 8 — Ordenação Bolha (Bubble Sort)

Complexidade: O(n2)

Lógica do algoritmo:
Percorre a coleção múltiplas vezes. A cada passagem pelo laço interno, compara elementos adjacentes e os troca de lugar (swap) se estiverem na ordem errada (o maior "flutua" para o final do vetor). Repete o processo até que nenhum elemento precise ser trocado.

Justificativa da complexidade:
Contém dois loops aninhados onde o loop externo dita o número de passagens pela lista (n iterações) e o loop interno varre os elementos não ordenados restantes (n−i−1 iterações). No pior caso (lista ordenada em ordem inversa), serão feitas O(n2) comparações e trocas.



Exercício 9 — Produto de Matrizes

Complexidade: O(n3)

Lógica do algoritmo:
Realiza a multiplicação algébrica clássica entre duas matrizes quadradas de dimensões n×n. Ele inicializa uma matriz de resultados preenchida com zeros e calcula cada célula através do produto escalar das linhas da matriz A pelas colunas da matriz B.

Justificativa da complexidade:
O algoritmo necessita de três loops for profundamente aninhados, cada um variando de 0 até n. O loop mais externo controla as linhas, o intermediário as colunas e o mais interno realiza a acumulação dos produtos das coordenadas, totalizando exatamente n×n×n=n3 operações.



Exercício 10 — Merge Sort

Complexidade: O(nlogn)

Lógica do algoritmo:
Algoritmo de ordenação baseado no paradigma de "Dividir e Conquistar". Divide recursivamente o vetor ao meio até obter subvetores de tamanho 1 (já ordenados). Em seguida, mescla (merge) esses subvetores de forma ordenada para reconstruir a estrutura final.

Justificativa da complexidade:
A árvore de divisão possui uma altura máxima de log2​n níveis. Em cada nível da árvore recursiva, a operação de fusão (reunir as metades) percorre todos os elementos combinados daquele nível, realizando um trabalho linear de tempo O(n). Multiplicando a profundidade pelo custo de cada nível, obtém-se O(nlogn).