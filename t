for question in $( ls ./data/q/*.txt ); do
  answer=./data/a/${question##*/}

  time target/release/stamp-art < ${question} > ${answer}
done
