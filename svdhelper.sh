# from gitub.com/dhoove/tixml2svd instructions:

# mkdir tmp
# cp -r /media/sf_ti/ccsv8/ccs_base/common/targetdb/devices tmp
# cp -r /media/sf_ti/ccsv8/ccs_base/common/targetdb/Modules tmp
# find tmp -type f -exec dos2unix \{\} \;
# cd tmp
# find devices -name \*.xml -exec sed -i '1s/^\xEF\xBB\xBF//' \{\} \;

# Now you can run the script here to generate the PAC for each board that you have taken from CCSV

find ./devices -type f -name "*.xml" | while read line; do
    BOARD=`echo $line | grep -o -P '(?<=devices/).*(?=.xml)'`
    tixml2svd -z -i $line > `echo $BOARD.svd`
    svd2rust -i `echo $BOARD.svd`
    
    mkdir $BOARD #&& cd $BOARD
    form -i lib.rs -o `echo $BOARD/src` && rm lib.rs
    
done
