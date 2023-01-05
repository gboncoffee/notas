#include<stdio.h>
#include<stdlib.h>
#include<curses.h>
#include<errno.h>

int main() {
    initscr();

    /*
     * Há uma parte da biblioteca considerada "low level". Entre as funções
     * úteis dela estão:
     */

    /*
     * curs_set(int), para setar o modo do cursor: 0 é invisível, 1 é pouco
     * visível e 2 é completamente visível. Retorna ERR se não for possível, e o
     * último estado do cursor se for possível.
     */

    endwin();
    return 0;
}
