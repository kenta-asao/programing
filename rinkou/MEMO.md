# ２．１．Introduction
 組み合わせをSATで解く前に，大抵の場合，連言標準形（CNF）に変換（符号化）する必要がある：節の連言で，それぞれの節はリテラルの選言であり・それぞれのリテラルはブール変数かその否定である。
 CNFの利点は，非常にシンプルな形式で，かんたんなアルゴリズムの実装や共通のファイル形式が可能になることがある。
 残念ながら，ほとんどの問題の符号化の方法はいくつかあり，それらの中から選択するための明確なガイドラインはほとんどないが，符号化の選択はSATソルバーの選択と同じくらい重要である。

 この章では，CNF符号化の理論的，実証的なCNF符号化の研究について説明する。
 2.2節では，命題論理の式からCNF（および3-CNF）へ変換する技術，特に，Tseitin符号化とその変種，外包的で内包的な制約の符号化，およびCNFのためのDIMACSファイル形式について説明する。
 2.3節では，CNF符号化の事例研究を紹介し，このセクションの技術や他のモデリングの側面を説明します。
 2.4節では，ある符号化が別の符号化よりも優れていることについて議論する。
 最後に，2.5節がこの章を締めくくる。
 
 この章で言及されたトピックのいくつかは，この本の他の場所でより詳細に取り扱われているが，議論を補助するために，簡潔に扱う。
 読みやすさのために，SATの符号化の例から一部，詳細を省略するだろう。
 特に，　連言m,j=1（選言n i=1 vij）の代わりに単純に選言i vijと書く。
 また，変数iをその範囲にわたって定量化し，自由変数jをその範囲にわたって定量化することによって節の連言を形成することは，読者に委ねる。
 詳細については，以下で引用されているさまざまな論文を参照しなさい。
  
  ### 単語
  - conjunctive normal form（CNF）：連言標準形<br>
  - conjunction：連言<br>
  - disjunction：選言<br>
  - clause：節<br>
  - propositional logic：命題論理<br>
  - constraints：制約<br>
  - quantify：定量化する<br>
  - encoding：符号化<br>
  - variable：変数<br>
   
# 2.2.Transformation to CNF
 - 命題論理式はいくつかの方法でCNFに変換できるが，莫大な構造情報の詳細を失う可能性がある。
 - $A \land B$のような意味を持つ構造の情報

## 2.2.1．Transformation　by Boolean algebra
 $(a \Rightarrow (c \land d)) \lor (b \Rightarrow (c \land e))$<br>
 $((a \Rightarrow c) \land (a \Rightarrow d)) \lor ((b \Rightarrow c) \land (b \Rightarrow e))$　（分配律）<br>
 $(((a \Rightarrow c) \land (a \Rightarrow d)) \lor (b \Rightarrow c)) (((a \Rightarrow c) \land (a \Rightarrow d)) \lor (b \Rightarrow e))$　（分配律）<br>
 $((a \Rightarrow c) \lor (b \Rightarrow c)) \land ((a \Rightarrow d) \lor (b \Rightarrow c)) \land ((a \Rightarrow c) \lor (b \Rightarrow e)) \land ((a \Rightarrow d) \lor (b \Rightarrow e)) $　（分配律）<br>
 $((a \Rightarrow c) \lor (b \Rightarrow c))\land ((a \Rightarrow c) \lor (b \Rightarrow e)) \land ((a \Rightarrow d) \lor (b \Rightarrow c)) \land ((a \Rightarrow d) \lor (b \Rightarrow e))$　（結合律）<br>
 $(\overline{a} \lor \overline{b} \lor c) \land (\overline{a} \lor \overline{b} \lor c \lor e) \land (\overline{a} \lor \overline{b} \lor c \lor d) \land (\overline{a} \lor \overline{b} \lor d \lor e)$　（巾等律）<br>
 $(\overline{a} \lor \overline{b} \lor c) \land (\overline{a} \lor \overline{b} \lor d \lor e)$　（吸収律）
 
## 2.2.2．Transformation by Tseitin encoding
 $f_1 \Leftrightarrow (c \land d)$<br>
 $(f_1 \Rightarrow (c \land d)) \land ((c \land d) \Rightarrow f_1)$　（定義）<br>
 $(\overline{f_1} \lor (c \land d)) \land (f_1 \lor \overline{(c \land d)})$<br>
 $(\overline{f_1} \lor (c \land d)) \land (f_1 \lor (\overline{c} \lor \overline{d}))$　（ド・モルガン）<br>
 $(\overline{f_1} \lor c) \land (\overline{f_1} \lor d) \land (\overline{c} \lor \overline{d} \lor f_1)$　（分配律・結合律）<br>

 $f_2 \Leftrightarrow (c \land e)$<br> 
 $(\overline{f_2} \lor c) \land (\overline{f_2} \lor e) \land (\overline{c} \lor \overline{e} \lor f_2)$　（式変形は $f_1$ と同じ）

 $(a \Rightarrow (c \land d)) \lor (b \Rightarrow (c \land e))$<br>
 $(a \Rightarrow f_1) \lor (b \Rightarrow f_2)$　（代入）<br>
 $(\overline{a} \lor f_1) \land (\overline{b} \lor f_2)$　（ $ \lor $ の間違い）<br>

 $f_3 \Leftrightarrow (\overline{a} \lor f_1)$<br>
 $f_4 \Leftrightarrow (\overline{b} \lor f_2)$<br>
 $(\overline{f_3} \lor \overline{a} \lor f_1) \land (a \lor f_3) \land (\overline{f_1} \lor f_3) \land (\overline{f_4} \lor \overline{a} \lor f_2) \land (b \lor f_4) \land (\overline{f_2} \lor f_4)$<br>
 $(f_3 \lor f_4)$<br>
 $f_5 \Leftrightarrow (f_3 \lor f_4)$<br>
 $(\overline{f_5} \lor f_3 \lor f_4) \land (\overline{f_3} \lor f_5) \land (\overline{f_4} \lor  f_5)$　（式変形は $f_1$ と同じ）<br>

## 2.2.3 Transformation from CNF to 3-CNF
 $c$ を節とし， $len(c)$ を $c$ のリテラルの数を返す関数とする。
<br>
任意の長さの節 $c$ を長さ３の節の連言に変換する関数 $T$ を以下のように定義する。
 $$
  T(c)= \left\lbrace
    \begin{align*}
      &\begin{split}
      (z_1 \lor z_2 \lor y_{i,1}) \land (\overline{y_{i,1}} \lor z_3 \lor y_{i,2}) \land (\overline{y_{i,2}} \lor z_4 \lor y_{i,3}) \land ... \land (\overline{y_{i,k-3}} \lor z_{k-1} \lor z_k)
      \end{split}&len(c)>3\\
      &\begin{split}
      c
      \end{split}&len(c) = 3\\
      &\begin{split}
      (z_1 \lor z_2 \lor y_{i,1}) \land (z_1 \lor z_2 \lor \overline{y_{i,1}})
      \end{split}&len(c) = 2\\
      &\begin{split}
      (z_1 \lor y_{i,1} \lor y_{i,2}) \land (z_1 \lor \overline{y_{i,1}} \lor y_{i,2}) \land (z_1 \lor y_{i,1} \lor \overline{y_{i,2}}) \land (z_1 \lor \overline{y_{i,1}} \lor \overline{y_{i,2}}) 
      \end{split}&len(c) = 1
    \end{align*}
  \right.
 $$

 $c \Leftrightarrow T(c)$ を証明する<br>

 $len(c)=1$ のとき<br>
 $z_1$ <br>
 $\equiv (z_1 \lor y_{i,1}) \land (z_1 \lor \overline{y_{i,1}})$ <br>
 $\equiv (z_1 \lor y_{i,1} \lor y_{i,2}) \land (z_1 \lor y_{i,1} \lor \overline{y_{i,2}}) \land (z_1 \lor \overline{y_{i,1}} \lor y_{i,2}) \land (z_1 \lor \overline{y_{i,1}} \lor \overline{y_{i,2}})$ <br>
 $\equiv (z_1 \lor y_{i,1} \lor y_{i,2}) \land (z_1 \lor \overline{y_{i,1}} \lor y_{i,2}) \land (z_1 \lor y_{i,1} \lor \overline{y_{i,2}}) \land (z_1 \lor \overline{y_{i,1}} \lor \overline{y_{i,2}})$ <br>
 よって， $z_1$ と $(z_1 \lor y_{i,1} \lor y_{i,2}) \land (z_1 \lor \overline{y_{i,1}} \lor y_{i,2}) \land (z_1 \lor y_{i,1} \lor \overline{y_{i,2}}) \land (z_1 \lor \overline{y_{i,1}} \lor \overline{y_{i,2}})$ は同値であるので，$len(c)=1$ のとき $c \Leftrightarrow T(c)$ が成り立つ。

 $len(c)=2$ のとき<br>
 $(z_1 \lor z_2)$ <br>
 $\equiv (z_1 \lor z_2 \lor y_{i,1}) \land (z_1 \lor z_2 \lor \overline{y_{i,2}})$ <br>
 よって， $(z_1 \lor z_2)$ と $(z_1 \lor z_2 \lor y_{i,1}) \land (z_1 \lor z_2 \lor \overline{y_{i,2}})$ は同値であるので， $len(c)=2$ のとき $c \Leftrightarrow T(c)$ が成り立つ。<br>

 $len(c)=3$ のとき<br>
 自明<br>

 $len(c)>3$ のとき<br>

 ( $\Rightarrow$ )<br>
 $z_1 \lor z_2$ が真のとき<br>
 すべての $y_{i,k} (k=1,2,...,len(c)-3)$ が0のときに $c \Rightarrow T(c)$ が成り立つ。<br>

 $z_1 \lor z_2$ が偽のとき<br>
 $z_j (j=3,4,...len(c))$ を真とすると，<br>
 $y_{i,k} (k=1,2,...,j-2)$ が1かつ， $y_{i,k} (k=j-2,...,len(c))$が0のときに $c \Rightarrow T(c)$ が成り立つ。<br>

 よって， $c \Rightarrow T(c)$ が成り立つ。<br>

 ( $\Leftarrow$ )<br>
 背理法で証明する。<br>

 （仮定） $T(c)$ が真のとき， $c$ は偽であるとする。<br>

 $c$ は偽であるとき，<br>
 $\overline{c} = \overline{z_1 \lor z_2 \lor ... \lor z_{len(c)}}$<br>
 $=\overline{z_1} \land \overline{z_2} \land ... \land \overline{z_{lec(c)}}$ （ド・モルガン則）<br>
 が真となるので，$z_k (z=1,2,...len(c))$ はすべて， $0$ となる。<br>
 よって， $T(c) = (z_1 \lor z_2 \lor y_{i,1}) \land (\overline{y_{i,1}} \lor z_3 \lor y_{i,2}) \land (\overline{y_{i,2}} \lor z_4 \lor y_{i,3}) \land ... \land (\overline{y_{i,k-3}} \lor z_{k-1} \lor z_k)$は $z_k (z=1,2,...len(c))$ がすべて， $0$ なので，<br>
 $y_{i,1} \land \overline{y_{i,1}} \land y_{i,2} \land \overline{y_{i,2}} \land ... \land \overline{y_{i,k-3}}$を満たす必要がある。<br>
 これは矛盾する。<br>
 よって， $T(c)$ が真のとき， $c$ は真である。<br>
 よって， $c \Leftarrow T(c)$ が成り立つ。

 よって， $len(c) > 3$ のとき， $c \Leftrightarrow T(c)$ 

 ### 単語
  - in one's own right：自分だけで、他に依存せずに、独立して
  - be known to be ~：~として知られている
  - expressive：表現豊かな
  
## 2.2.4 Extensional constraints in CNF
  ### 直接符号化

  | x | y |
  | - | - |
  | 0 | 0 |
  | 1 | 1 |
  | 2 | 2 |
  | 3 | 3 |

  $i$ を点 $x,y,z$ ， $a$ を取りうる値とする。<br>
  $p_{k,a} \Leftrightarrow v_k=a (k=x,y,z)$<br>

  at-least-one節<br>
  $p_{i,0} \lor p_{i,1} \lor p_{i,2} \lor p_{i,3}$ $(i \in \{x,y,z\})$ <br>

  $\bigwedge_{i \in \{x,y,z\}}$

  at-most-one節<br>
  $\overline{p_{i,0}} \lor \overline{p_{i,1}}$<br>
  $\overline{p_{i,0}} \lor \overline{p_{i,2}}$<br>
  $\overline{p_{i,0}} \lor \overline{p_{i,3}}$<br>
  $\overline{p_{i,1}} \lor \overline{p_{i,2}}$<br>
  $\overline{p_{i,1}} \lor \overline{p_{i,3}}$<br>
  $\overline{p_{i,2}} \lor \overline{p_{i,3}}$<br>

  矛盾節<br>
  $\overline{p_{x,0}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,0}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,0}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,2}}$<br>

  $\overline{p_{x,0}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,0}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,0}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,3}}$<br>

  $\overline{p_{x,0}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,0}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,0}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,1}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,2}} \lor \overline{p_{y,3}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,0}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,1}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,2}}$<br>
  $\overline{p_{x,3}} \lor \overline{p_{y,3}}$<br>
 ### 単語