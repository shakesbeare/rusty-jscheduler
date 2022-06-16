scheduleSlots(instrAvail, students) //Returns an array with all possible schedule solutions
                                    //instrAvail is an array of all minutes in the instructor's availability
                                    //students is an array of student availabilities with same structure as instrAvail
    studentsSlots[0..students.length-1]
    for i = 0 to students.length-1
        Find all possible slots (represented as arrays of minutes) that work for student i
        Store as an array of slots in studentsSlots[i]
    possibleTakenAvails[] //Array of ordered pairs: 
                          //(takenAvail - array of minutes taken, students - array of student indices in chronological order)
                          //Eventually possibleTakenAvails will represent all possible schedule solutions
    possibleTakenAvails[0] = (empty array, empty array)
    for i = 0 to students.length-1
        r[]
        for j = 0 to possibleTakenAvails.length
            for k = 0 to studentsSlots[i].length
                Combine possibleTakenAvails[j].takenAvail and studentsSlots[i][k] into new array x
                conflicts = false
                Set conflicts to true if x contains any duplicates
                if !conflicts
                    y[]
                    y = possibleTakenAvails[j].students
                    y.append(i)
                    r.append((x, y))
        possibleTakenAvails = r
    return possibleTakenAvails