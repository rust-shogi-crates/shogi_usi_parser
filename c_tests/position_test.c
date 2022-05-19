

#include <assert.h>
#include <shogi_usi_parser.h>

const unsigned char *STARTPOS = (const unsigned char *) "sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

int main() {
    Position *pos = Position_parse_usi_slice(STARTPOS);
    assert (pos != 0);
    return 0;
}
