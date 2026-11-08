"""Pygments lexers for the Apple-1 manuals: Integer BASIC, Woz Monitor sessions."""
import re
from pygments.lexer import RegexLexer, bygroups, include
from pygments.token import (Comment, Keyword, Name, Number, Operator, Punctuation,
                            String, Text, Whitespace, Generic)

BASIC_STMT = ("REM LET DIM TAB END INPUT FOR TO STEP NEXT RETURN GOSUB GOTO IF THEN "
              "PRINT POKE CALL LIST RUN CLR NEW AUTO MAN DEL HIMEM LOMEM SCR CON").split()
BASIC_FUNC = "PEEK RND SGN ABS LEN".split()
BASIC_OPS = "NOT AND OR MOD".split()


class IntegerBasicLexer(RegexLexer):
    name = "Apple-1 Integer BASIC"
    aliases = ["basic", "intbasic"]
    flags = re.IGNORECASE
    tokens = {
        "root": [
            (r"^(\s*)(\d+)", bygroups(Whitespace, Name.Label)),
            (r"(REM)(.*)$", bygroups(Comment, Comment)),
            (r'"[^"\n]*"?', String),
            (r"\b(%s)\b" % "|".join(BASIC_FUNC), Name.Builtin),
            (r"\b(%s)\b" % "|".join(BASIC_OPS), Operator.Word),
            (r"(%s)" % "|".join(sorted(BASIC_STMT, key=len, reverse=True)), Keyword),
            (r"[A-Z][0-9]?\$?", Name.Variable),
            (r"\d+", Number),
            (r"<=|>=|<>|[-+*/^=<>#]", Operator),
            (r"[(),;:]", Punctuation),
            (r"\s+", Whitespace),
            (r".", Text),
        ]
    }


class WozMonLexer(RegexLexer):
    """Monitor / ACI session: lines starting with '> ' are typed by the user."""
    name = "Woz Monitor session"
    aliases = ["mon", "woz"]
    tokens = {
        "root": [
            (r"^(> ?)(.*)$", bygroups(Generic.Prompt, Generic.Strong)),
            (r"#.*$", Comment),
            (r"^[0-9A-F]{4}:", Name.Label),
            (r"[\\*@]", Generic.Prompt),
            (r".", Text),
            (r"\n", Whitespace),
        ]
    }
