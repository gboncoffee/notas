#include<stdio.h>
#include<stdlib.h>
#include<curses.h>

/*
 * ncurses é uma biblioteca para criar programas de terminal com interface que
 * imita programas de GUI.
 *
 * Para utilizá-la, basta ter instalado a biblioteca,
 * incluir o curses.h e passar a bandeira -lncurses
 * para o compilador.
 */

int basico() {
    // a biblioteca exige que o locale esteja definido. Pode-se escolher, POSIX
    // é exemplo.
    setlocale(LC_ALL, "POSIX");

    /*
     * Inicia-se a biblioteca com initscr(). Quando for tirar o terminal do
     * controle dela, deve-se chamar endwin(). Por isso, sempre chama-se
     * endwin() no final do programa.
     * */
    atexit(quit_window_and_exit);
    initscr();

    // Desliga o buffered IO baseado em newlines.
    cbreak();
    // Desliga o echoing (não mostra o que está sendo digitado).
    noecho();
    // Desliga o flush em interrupt, o que poderia fazer a biblioteca
    // dessincronizar com a tela real.
    intrflush(stdscr, FALSE);
    // Ativa o retorno de escape sequences tratadas. Sem isso, o programa teria
    // de parseá-las sozinho.
    keypad(stdscr, TRUE);

    /*
     * A biblioteca define as constantes TRUE e FALSE como 1 e 0.
     *
     * Além disso, LINES e COLS são definidas com o tamanho do terminal. stdscr
     * é a tela normal (um programa pode manipular mais de uma).
     *
     * Funções prefixadas com p precisam que se especifique um pad, e funções
     * prefixadas com w, uma WINDOW*. Funções sem prefixo usam a stdscr. Funções
     * prefixadas com mv precisam de um x e um y e pressupõe uma call para
     * move(). y é sempre LINHA e x COLUNA. O canto superior esquerdo é (0, 0).
     *
     * Funções prefixadas com mvw recebem sempre uma window, um x e um y, nessa
     * ordem.
     *
     * win e pad são sempre pointers para WINDOW.
     *
     * A maioria dos tipos estão definidos em curses.h, porém alguns estão em
     * term.h.
     */

    /*
     * Na configuração normal "ncurses", cada cell é guardada como um chtype.
     *
     * A configuração wide "ncursesw" suporta carácteres multibyte:
     * - cchar_t: guarda carácteres de 4 bytes e atributos. Cada cell é guardada
     *   como um.
     * - wchar_t: guarda um "wide" character. 4 bytes.
     * - wint_t: guarda um wchar_t ou WEOF - diferentes, porém de mesmo tamanho.
     *
     * A biblioteca wide possui funções com "_w" no nome. E.g., waddch na wide é
     * wadd_wch.
     */

    /*
     * Funções podem retornar ERR como erro e qualquer outra coisa como sucesso,
     * a depender da função. Geralmente, tratam NULL como erro. Funções que
     * retornam pointers retornam NULL como erro.
     */

    endwin();
	
	return(0);
}
