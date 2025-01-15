<script setup lang="ts">
import { Calendar } from '@/components/ui/calendar'
import OrganizerLayout from '@/layouts/OrganizerLayout.vue'
import { useOrganizerStore } from '@/stores/organizer';
import { CalendarDate, type DateValue, getLocalTimeZone, today } from '@internationalized/date'
import { type Ref, ref, computed } from 'vue'
import Task from '@/components/organizer/tasklist/Task.vue'
</script>
<template>
    <OrganizerLayout>
		<div class="border w-full h-full">
            <div class="grid grid-cols-7 grid-rows-[24] overflow-scroll h-full w-full">
                <div class="row-span-1 col-span-1 border"></div>
            </div>
        </div>
	</OrganizerLayout>
</template>