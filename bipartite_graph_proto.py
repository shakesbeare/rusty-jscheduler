class Match:
    def __init__(self, time_node, stud_node):
        self.timeslot_node = time_node
        self.student_node = stud_node

    def __repr__(self):
        return f"({self.timeslot_node}, {self.student_node})"

    def __iter__(self):
        self.n = 0
        return self

    def __next__(self):
        if self.n == 0:
            return self.timeslot_node
        elif self.n == 1:
            return self.student_node
        else:
            raise StopIteration

    def __getitem__(self, arg):
        if arg == 0:
            return self.timeslot_node
        elif arg == 1:
            return self.student_node


class Node:
    def __init__(self):
        self.connections = []
        self.num_connections = 0
        self.matched = False
        self.matched_partner = None

    def connect(self, *others):
        for other in others:
            self.connections.append(other)
            self.num_connections = len(self.connections)

            other.connections.append(self)
            other.num_connections = len(self.connections)

    def choose_next(self):  # returns false if no valid connection
        for connection in self.connections:
            if not connection.matched:
                return connection
        return None


class Student(Node):
    def __init__(self, duration, name):
        super().__init__()
        self.duration = duration
        self.name = name

    def __str__(self):
        return f"{self.name}"

    def __repr__(self):
        return self.__str__()


class Timeslot(Node):
    def __init__(self, start_time):
        super().__init__()
        self.start_time = start_time

    def __str__(self):
        return f"{self.start_time}"

    def __repr__(self):
        return self.__str__()


class BipartiteGraph:
    def __init__(self, timeslots, students):
        self.timeslots = timeslots
        self.students = students
        self.nodes = timeslots + students
        self.solution = []

    def has_unmatched_nodes(self):
        for node in self.nodes:
            if not node.matched:
                return True

        return False

    def match(self, timeslot_node, student_node):
        timeslot_node.matched = True
        timeslot_node.matched_partner = student_node
        student_node.matched = True
        student_node.matched_partner = timeslot_node
        self.solution.append(Match(timeslot_node, student_node))

    def get_smallest_node(self):
        smallest_node = None
        for node in self.nodes:
            if smallest_node is None:
                smallest_node = node
            elif node.num_connections < smallest_node.num_connections:
                smallest_node = node

        return smallest_node

    def find_matching(self):
        current_node = self.get_smallest_node()
        while self.has_unmatched_nodes():
            if current_node is None:
                break

            next_node = current_node.choose_next()
            if next_node is None:
                break
            self.match(current_node, next_node)
            current_node = next_node.choose_next()

        if len(self.solution) == len(self.students):
            # the solution contains all students
            self.solution.sort(key=lambda x: x[0].start_time)
            return self.solution

        match_index = -1
        last_match = self.solution[match_index]
        current_node = last_match[0]

        while self.has_unmatched_nodes():
            if current_node is None:
                break

            if current_node.matched_partner:
                if (
                    len(current_node.matched_partner.connections) == 1
                    and current_node.matched_partner.matched
                ):
                    match_index -= 1
                    last_match = self.solution[match_index]
                    current_node = last_match[0]
                    continue

            if not current_node.matched:
                next_node = current_node.choose_next()

                if next_node is None:
                    continue

                self.match(current_node, next_node)
                current_node = current_node.matched_partner.choose_next()
                continue

            new_connection_index = (
                current_node.connections.index(current_node.matched_partner) + 1
            )
            old_partner = current_node.matched_partner
            old_partner.matched = False
            old_partner.matched_partner = None

            new_partner = current_node.connections[new_connection_index]

            del self.solution[match_index]
            self.match(current_node, new_partner)
            current_node = old_partner.choose_next()

        self.solution.sort(key=lambda x: x[0].start_time)
        return self.solution


def create_graph():
    # Create timeslot nodes
    timeslots = []
    for x in range(5):
        node = Timeslot(x)
        timeslots.append(node)

    students = []
    for letter in ["a", "b", "c", "d", "e"]:
        node = Student(duration=1, name=letter)
        students.append(node)

    # connect nodes
    timeslots[0].connect(students[0], students[2])
    timeslots[1].connect(students[0], students[1])
    timeslots[2].connect(students[0], students[1], students[3])
    timeslots[3].connect(students[4])
    timeslots[4].connect(students[3], students[4])

    return BipartiteGraph(timeslots, students)


def main():
    graph = create_graph()
    solution = graph.find_matching()
    print(solution)


if __name__ == "__main__":
    main()
