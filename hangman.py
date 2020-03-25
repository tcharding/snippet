#!/usr/bin/python3
import sys

# Set the word to guess here.
word = "balloons";

guessed = "";
num_lives = 11;


def main():
   while True:
      if game_is_over():
         display()
         print_result()
         sys.exit()
         
      display()
      read_guess()


def game_is_over():
   return person_is_dead() or word_is_guessed()


def display():
   clear_screen()
   print('Welcome to hangman, good luck!')
   display_person()
   display_word()
   display_guessed()


def read_guess():
   global guessed
   global num_lives

   nl()
   guess = input("Guess a letter: ")

   guess.rstrip()
   if len(guess) > 1:
      print("Just guess a single letter please")
      read_guess()

   if guess not in guessed:
      if guess not in word:
         num_lives -= 1;
         
      guessed = guessed + guess


def person_is_dead():
   return num_lives <= 0


def word_is_guessed():
   for c in word:
      if c not in guessed:
         return False

   return True

   
def display_person():
   global num_lives;

   if num_lives == 11:
      print("\n\
                    \n\
                    \n\
                    \n\
                    \n\
                    \n\
                    \n\
                    \n\
                    \n\
      ")

   if num_lives == 10:
      print("\n\
                    \n\
                    \n\
                    \n\
                    \n\
                    \n\
                    \n\
                    \n\
       -------      \n\
      ")

   if num_lives == 9:
      print("\n\
                    \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 8:
      print("\n\
          ______    \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 7:
      print("\n\
          ______    \n\
          |/        \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 6:
      print("\n\
          ______    \n\
          |/   |    \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 5:
      print("\n\
          ______    \n\
          |/   |    \n\
          |    O    \n\
          |         \n\
          |         \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 4:
      print("\n\
          ______    \n\
          |/   |    \n\
          |    O    \n\
          |    |    \n\
          |    |    \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 3:
      print("\n\
          ______    \n\
          |/   |    \n\
          |    O    \n\
          |   /|    \n\
          |    |    \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 2:
      print("\n\
          ______    \n\
          |/   |    \n\
          |    O    \n\
          |   /|\   \n\
          |    |    \n\
          |         \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 1:
      print("\n\
          ______    \n\
          |/   |    \n\
          |    O    \n\
          |   /|\   \n\
          |    |    \n\
          |   /     \n\
          |         \n\
       -------      \n\
      ")

   if num_lives == 0:
      print("\n\
          ______    \n\
          |/   |    \n\
          |    O    \n\
          |   /|\   \n\
          |    |    \n\
          |   / \   \n\
          |         \n\
       -------      \n\
      ")


def display_word():
   nl()
   print("    WORD: ", end='')
   for c in word:
      if c in guessed:
         print(" %s" % c, end='')
      else:
         print(" _", end='')

   nl()
   nl()


def display_guessed():
   global guessed
   
   print("Letters guessed so far: '", end='')
   for c in guessed:
      print(" %s" % c, end='')
   print(" '")


def print_result():
   nl()
   if person_is_dead():
      print("Unlucky - you hanged.")
      nl()
      print("By the way, the word was '%s'." % word)
      nl()
   else:
      print("You win, well done!")
   

def clear_screen():
   print("--------- take a turn -------")
   print("\033[H\033[J")
   nl()
   nl()
   

def nl():
   print("")


if __name__ == "__main__":
    main()
     
