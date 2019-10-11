for question in $( ls ./data/q/*.txt ); do
  answer=./data/a/${question##*/}

  echo -- ${question##*/}
  time target/release/stamp-art < ${question} > ${answer}
done
