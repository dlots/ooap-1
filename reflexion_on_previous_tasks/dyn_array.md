# Dyn Array 
Отличия от эталонного решения:
* Статусы методов
  - В эталонном решении - _Ок_ или _Out of bounds_
  - Я решил добавить дополнительный статус _"массив пуст"_ для методов, чтобы можно было отличить ошибку пустого массива от ошибки _"index out of bounds"_.
  - В ретроспективе, в этом мало смысла, так как пустоту массива можно проверить методом **size()**/**len()**, а успех метода определяется по 2-м статусам
* Пред- и пост-условия
  - В эталонном решении, у методов предусловие "Индекс находится в допустимых пределах"
  - У методов с предусловием в моем решении предусловие "Массив не пуст". В этом предусловии не учитывается валидность индекса, по которому происходит обращение (при этом проверка производится, и имеется отдельный статус для такого случая).
  - Я не совсем верно понял понятие предусловия, т.к. я считал, что предусловием является исключительно состояние объекта на момент вызова метода, а свойства аргументов метода в него не входят.
  - Несмотря на это различие, в моем решении все методы, которые нуждаются в постусловии, имеют его.
  - Все пост-условия соответствуют эталонному решению.
* Способ доступа к элементам
  - В эталонном решении индекс элемента напрямую передается в методы.
  - В моем решении используется курсор, как в списке. Имеется дополнительный метод set_cursor(i), который проверяет, пуст ли массив, и находится ли индекс в допустимых границах массива. Курсор всегда указывает на какой-либо индекс, если массив не пуст. Начальный индекс - 0. Остальные операции выполняются с текущим индексом.
  - В ретроспективе, курсор не выглядит необходимым для динамического массива. В отличие от, например, списка, от пользователя массива не нужно скрывать особенности реализации структуры, например, узлы. Также, прямая индексация является естественной и привычной при работе с массивами.
* Набор операций
  - Реализованный мной массив поддерживат те же операции, что и массив из эталонного решения, кроме put_right(). Этот метод может быть полезен, когда требуется, чтобы элемент i остался на своем месте.
  - Остальные операции одинаковы, но некоторые называются по разному:
    * put = replace
    * put_left = insert
    * size = len
* В определении АТД, не выделил явно запросы и команды

В итоге, моя имплементация почти полностью соответствует эталонному решению. Основные ошибки - неправильное пред-условие и курсор, который для массива будет скорее лишним.