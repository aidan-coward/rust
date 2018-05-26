#! /bin/bash

CMUS=`cmus-remote -Q | grep status | grep -v comment | cut --delimiter=\  -f2`

if [[ $CMUS == paused ]] || [[ $CMUS == playing ]]
then
	echo "1"
else 
	echo "0"
fi
