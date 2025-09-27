{- |
      Module      : Main
      Description : Reads program arguments and
                    invokes compiler routines.
      Copyright   : © Multislot Inc, 2022
      License     : All Rights Reserved
      Maintainer  : 10951848+CubOfJudahsLion@users.noreply.github.com
      Stability   : experimental
      Portability : POSIX
      
      SWIRL is a WASM Web media rendering library. This
      utility automates code generation for loading
      assets and displaying them on screen through the
      simple SADL language.
 -}

module Main where

import Control.Monad (forM_)
import Data.List (intercalate, foldl')
import GHC.Exception.Type (Exception)
import System.Environment (getArgs, getExecutablePath)
import System.IO
import System.IO.Error (catchIOError)
import Language.SADL.Lexer (customScanTokens)


tokenDump :: String -> String -> IO ()
tokenDump i o = do
  contents <- catchIOError (Just <$> readFile i) (const $ return Nothing)
  case contents of
    Nothing ->
      hPutStrLn stderr $ "Could not read file \"" ++ i ++ "\"."
    Just s  -> do
      forM_ (customScanTokens s) (putStrLn . show)

  
main :: IO ()
main = do
  args <- getArgs
  case args of
    [i]     | not $ null i
            -> tokenDump i $ targetFileName i
    [i, o]  | not (null i) && not (null o)
            -> tokenDump i o
    _       -> do execPath <- getExecutablePath
                  putStrLn $ "usage: " ++ execPath ++ "in-file [out-file]"
  where
    targetFileName :: String -> String
    targetFileName s = case break ('.' ==) $ reverse s of
      (s', "")  -> reverse s' ++ ".rs"
      (_, s')   -> reverse s' ++ "rs"
