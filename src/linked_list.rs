#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RightStatus {
    Nil,    // never called right()
    Ok,     // last right() call was successful
    Empty,  // last right() called on empty list
    NoNext  // last right() called when current node had no right neighbor
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RemoveStatus {
    Nil,    // never called remove()
    Ok,     // last remove() call was successful
    Empty,  // last remove() called on empty list
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReplaceStatus {
    Nil,    // never called replace()
    Ok,     // last replace() call was successful
    Empty,  // last replace() called on empty list
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GetStatus {
    Nil,    // never called get()
    Ok,     // last get() call was successful
    Empty,  // last get() called on empty list
}

// Definition of Abstract Data Type "LinkedList"
pub trait LinkedList<T> {
    /**
     * Constructor
    **/
    // Post-condition: a new empty list is created, the cursor is not defined
    fn new() -> Self;

    /**
     * Commands
    **/
    // Post-condition: the cursor points to the head of the list if exists
    fn head(&mut self);

    // Post-condition: the cursor points to the tail of the list if exists
    fn tail(&mut self);

    // Pre-condition: the list is not empty and the next node exists
    // Post-condition: the cursor points to the next node
    fn right(&mut self);

    // Post-condition: a new node is inserted after the current node
    // If the list was empty, the cursor points to the new node
    fn put_right(&mut self, value: T);

    // Post-condition: a new node is inserted after the current node
    // If the list was empty, the cursor points to the new node
    fn put_left(&mut self, value: T);

    // Pre-condition: the list is not empty
    // Post-condition: current node is deleted, the cursor points to the next
    // node if it exists, or to the previous node if it exists
    fn remove(&mut self);

    // Post-condition: all nodes are deleted. The cursor is not defined.
    fn clear(&mut self);

    // Post-condition: a new node is inserted at the end of the list
    // If the list was empty, the cursor points to a new node.
    fn add_tail(&mut self, value: T);

    // Pre-condition: the list is not empty
    // Post-condition: the value of the current node is updated
    fn replace(&mut self, value: T);

    // Post-condition: the cursor points to the node with the specified
    // value if such node is found after the current node
    fn find(&mut self, value: T);

    // Post-condition: all nodes with specified value are removed from the list
    fn remove_all(&mut self, value: T);

    /**
     * Queries
    **/
    // Pre-condition: the list is not empty
    fn get<'a>(&mut self) -> &'a T;

    fn size(&self) -> usize;

    fn is_head(&self) -> bool;

    fn is_tail(&self) -> bool;

    fn is_value(&self) -> bool;

    /**
     * Status queries
    **/
    fn get_right_status(&self) -> RightStatus;
    fn get_remove_status(&self) -> RemoveStatus;
    fn get_replace_status(&self) -> ReplaceStatus;
    fn get_get_status(&self) -> GetStatus;
}

// 2.2. Операция tail не сводима к другим операциям с точки зрения эффективной реализации,
// так как ее реализация через операцию right в худшем случае имеет сложность O(n).
// Эффективнее хранить указатель на "хвост" списка и обновлять его по необходимости.

// 2.3. Операция поиска всех узлов с заданным значением больше не нужна, так как она не
// совместима с концепцией курсора. Пользователь АТД больше не может работать с узлами
// напрямую, а курсор может указывать только на одну позицию в списке.

// TODO: реализовать связный список
// (сейчас не стал делать, т.к. хотел бы погрузиться в Rust, а указателей в нем нет;
// на тему связных списков есть отдельный Rust Book, изучу и реализую в свободное время)
