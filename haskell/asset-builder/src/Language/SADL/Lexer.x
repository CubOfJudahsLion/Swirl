{
{-|
    Module      : Language.SADL.Lexer
    Description : Lexer module for SADL
    Copyright   : © Multislot Inc., 2022
    License     : AllRightsReserved
    Maintainer  : 10951848+CubOfJudahsLion@users.noreply.github.com
    Stability   : experimental
    Portability : POSIX
    
    This is the tokenizer for SADL (SWIRL's
    Asset Description Language.) It's generated
    through @alex@.
-}

module Language.SADL.Lexer where

import Data.Word (Word16)
}

%wrapper "posn"

  $shell_char = [\< \> \# \% @ \$ \+ \* \? ! \, \; : \/ \\ \| \[ \] \{ \} \' \" \`]
  $file_char  = $printable # $shell_char
  
  @file_part  = \" $file_char+ \"
  @int        = 0 | [1-9] [0-9]*
  @float      = 0? . (0 ([0-9]* [1-9])? | [1-9] [0-9]* [1-9])?
              | [1-9] [0-9]* . (
  @ident      = [A-Z] [a-z A-Z 0-9]*
  @var        = [a-z] [a-z A-Z 0-9 _]*

tokens :-

  "//" .*         ;
  "--" .*         ;
  $white+         ;
  assets          { const . const Assets }
  graph           { const . const Graph }
  piv(ot)?        { const . const Pivot }
  sk(ew)?         { const . const Skew }
  sc(ale)?        { const . const Scale }
  rot(ate)?       { const . const Rotate }
  tr(ans(late)?)? { const . const Translate }
  @ident          { ($) . const Ident }
  @var            { ($) . const Var }
  ":"             { const . const Colon }
  "="             { const . const Equals }
  @file_part      { ($) . const (FilePart . tail . init) }
  ","             { const . const Comma }
  "+"             { const . const Plus }
  @int            { ($) . const (IntParam . read) }
  "("             { const . const LParen }
  ")"             { const . const RParen }
  "["             { const . const LBracket }
  "]"             { const . const RBracket }
  "..="           { const . const RangeInclusive }
  ".."            { const . const RangeExclusive }
  "{"             { const . const LBrace }
  "}"             { const . const RBrace }
  "->"            { const . const Arrow }

{
-- |  The lexeme set is small. That's sort of the point.
-- |  The grammar's also minimal, but the generated code
-- |  saves a lot of work.
data Token  = Assets          -- ^  The '@assets@' section keyword
            | Graph           -- ^  The '@graph@' section keyword
            | Pivot           -- ^  Setting a pivot     for a scene-graph element
            | Skew            -- ^  "       skewing     "   " "           "
            | Scale           -- ^  "       scaling     "   " "           "
            | Rotate          -- ^  "       rotation    "   " "           "
            | Translate       -- ^  "       translation "   " "           "
            | Colon
            | Ident String    -- ^  a __@PascalCase@__ name (i.e., a type or @enum@ identifier)
            | Var String      -- ^  a __@snake_case@__ or __@lowerCamelCase@__ name (a variable name,
                              --    for Rust or Haskell, respectively)
            | Equals
            | FilePart String -- ^  Part (or whole) of a filename
            | Comma
            | Plus
            | IntParam Word16 -- ^  An integer parameter
            | LParen
            | RParen
            | LBracket
            | RBracket
            | RangeInclusive  -- ^  Rust's `..=` operator, including the second argument
            | RangeExclusive  -- ^  `..` operator, excluding the second argument
            | LBrace
            | RBrace
            | Arrow           -- ^  A function arrow ("->"), used in both Haskell
                              --    and Rust to denote functions.
            -- | Eof
  deriving (Eq, Show)


customScanTokens :: String -> [Token]
customScanTokens str = go (alexStartPos,'\n',[],str)
  where go inp@(pos,_,_,str) =
          case alexScan inp 0 of
                AlexEOF -> [] -- [Eof]
                AlexError ((AlexPn _ line column),_,_,_) -> error $ "Bad token at line " ++ (show line) ++ ", column" ++ (show column)
                AlexSkip  inp' len     -> go inp'
                AlexToken inp' len act -> act pos (take len str) : go inp'
}
