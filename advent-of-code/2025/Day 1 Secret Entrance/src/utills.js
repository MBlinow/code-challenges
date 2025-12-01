export function getZeroesInRange(start, end){
    let zeroesFound = 0;

    let incrementor = start < end ? 1 : -1;
    for (let i = start; incrementor > 0 ? i <= end : i >= end; i += incrementor){
        if (i % 100 === 0){
            zeroesFound += 1;

            // Calculate remaining zeroes, no need to iterate further
            let remainingRange = Math.abs(end - i);
            let additionalZeroes = Math.floor(remainingRange / 100);
            zeroesFound += additionalZeroes;
            break;
        }
    }
    return zeroesFound;
}